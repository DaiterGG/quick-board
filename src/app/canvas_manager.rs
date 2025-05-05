use std::cmp::*;

use sdl2::{pixels::Color, rect::Rect, render::*, video::Window};

use super::{
    coords::*, history::History, pointer_state::PointerState, predefined::Id,
    style_display::DisplayState, texture_manager::*, tool_trait::*, ui_map::UIMap,
};

pub type StepId = usize;
pub struct CanvasManager {
    pub data: CanvasData,
    // ui_window_was_updated: bool,
    current_tool: ToolIdUsize,
    tools: Vec<Tool>,
}
/// data to pass to tools and save to config
///
/// * `transform`: WH of the canvas, XY texture offset
/// * `screen_pos`: position of the canvas inside draw_window
/// * `screen_zoom`: zoom multiplier
/// * `targeted_ui_texture`: id of the texture that is drawn to draw_window
pub struct CanvasData {
    pub transform: XYWH,
    pub screen_pos: XY,
    pub screen_zoom: f32,
    pub targeted_ui_texture: usize,
    pub targeted_ui_element: usize,
    pub history: History,
}
impl CanvasManager {
    pub fn new(texture_manager: &mut TextureManager, ui_map: &mut UIMap, window_id: usize) -> Self {
        let targeted_ui_texture = texture_manager.init_mut_texture(None);
        let draw_win_display = ui_map.display_mut(window_id).as_mut().unwrap().states_data
            [DisplayState::Idle as usize]
            .as_mut()
            .unwrap();
        *draw_win_display = draw_win_display.open_texture(targeted_ui_texture);
        Self {
            data: CanvasData {
                screen_pos: XY::new(100, 100),
                transform: XYWH::new(0, 0, 400, 300),
                history: History::new(),
                screen_zoom: 1.0,
                targeted_ui_texture,
                targeted_ui_element: window_id,
            },
            // ui_window_was_updated: true,
            current_tool: ToolId::Brush as usize,
            tools: Tool::init_all_tools(),
        }
    }
    pub fn update(
        &mut self,
        pointer: &PointerState,
        ui_map: &mut UIMap,
        canvas: &mut Canvas<Window>,
        textures: &mut TextureManager,
    ) {
        let draw_win_transform = ui_map.element(self.data.targeted_ui_element).transform;

        self.tools[self.current_tool].process_stroke(&mut self.data, pointer, canvas, textures);

        //draw to buffer
        let ui_tex_id = self.data.targeted_ui_texture;
        let ui_tex = textures.open_texture_mut(ui_tex_id);
        // if self.ui_window_was_updated {
        let src_dst = self.calc_canvas_pos(draw_win_transform);
        // this is just a buffer texture to be copied to draw_window directly
        ui_tex.src = Some(src_dst.1.to_rect());
        ui_tex.dst = Some(src_dst.1.to_rect());
        let _ = canvas.with_texture_canvas(&mut ui_tex.texture, |c| {
            c.set_draw_color(Color::RGB(20, 20, 20));
            let _ = c.fill_rect(src_dst.1.to_rect());
        });
        self.data
            .history
            .full_draw(canvas, textures, &self.data, src_dst.0, src_dst.1);
        // self.ui_window_was_updated = false;
        // }
    }
    pub fn change_tool(&mut self, tool_id: ToolId) {
        self.current_tool = tool_id as usize;
        // self.ui_window_was_updated = true;
    }
    pub fn add_zoom(&mut self, zoom_to_add: f32) {
        self.data.screen_zoom += zoom_to_add;
        // self.ui_window_was_updated = true;
    }
    pub fn move_canvas(&mut self, move_by: XY) {
        self.data.screen_pos = XY {
            x: self.data.screen_pos.x + move_by.x,
            y: self.data.screen_pos.y + move_by.y,
        };
        // self.ui_window_was_updated = true;
    }
    fn calc_canvas_pos(&mut self, ui_pos: XYWH) -> (XYWH, XYWH) {
        // let coords = XYWH::new(
        //     self.data.screen_pos.x,
        //     self.data.screen_pos.y,
        //     (self.data.transform.w as f32 * self.data.screen_zoom) as i32,
        //     (self.data.transform.h as f32 * self.data.screen_zoom) as i32,
        // );
        let to_ui_space = self
            .data
            .transform
            .transform_into(self.data.screen_zoom, self.data.screen_pos);

        let ui_overlap = to_ui_space.get_overlap(ui_pos);
        // let overlap_w = min(ui_pos.w, coords.x + coords.w) - max(0, coords.x);
        if ui_overlap.w <= 0 {
            return (XYWH::new(0, 0, 0, 0), XYWH::new(0, 0, 0, 0));
        }
        // let overlap.h = min(ui_pos.h, coords.y + coords.h) - max(0, coords.y);
        if ui_overlap.h <= 0 {
            return (XYWH::new(0, 0, 0, 0), XYWH::new(0, 0, 0, 0));
        }
        let dst = ui_overlap;

        let src = XYWH::new(
            -min(0, self.data.screen_pos.x),
            -min(0, self.data.screen_pos.y),
            (ui_overlap.w as f32 / self.data.screen_zoom) as i32,
            (ui_overlap.h as f32 / self.data.screen_zoom) as i32,
        );
        // let dst = Rect::new(
        //     ui_pos.x + max(0, coords.x),
        //     ui_pos.y + max(0, coords.y),
        //     (overlap_w) as u32,
        //     (overlap_h) as u32,
        // );
        // println!("src: {:?}, dst: {:?}", src, dst,);
        (src, dst)
    }
}
