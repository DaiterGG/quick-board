use std::cmp::*;

use sdl2::{rect::Rect, render::*, video::Window};

use super::{
    coords::*,
    history::History,
    pointer_state::PointerState,
    predefined::Id,
    style_display::DisplayState,
    texture_data::TextureData,
    texture_manager::{self, *},
    tool_trait::*,
    ui_map::UIMap,
};

pub type StepId = usize;
pub struct CanvasManager {
    data: CanvasData,
    ui_window_was_updated: bool,
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
    pub history: History,
}
impl CanvasManager {
    pub fn new(texture_manager: &mut TextureManager) -> Self {
        Self {
            data: CanvasData {
                screen_pos: XY::new(100, 100),
                transform: XYWH::new(0, 0, 400, 200),
                history: History::new(),
                screen_zoom: 1.0,
                targeted_ui_texture: texture_manager.init_mut_texture(None),
            },
            ui_window_was_updated: true,
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
        let draw_win_transform = ui_map.element(Id::MainCanvas as usize).transform;

        // set display to target texture buffer
        // TODO: do that in new()
        ui_map
            .display_mut(Id::MainCanvas as usize)
            .as_mut()
            .unwrap()
            .states_data[DisplayState::Idle as usize]
            .unwrap()
            .open_texture(self.data.targeted_ui_texture);

        if pointer.interacting_with.unwrap_or(0) == Id::MainCanvas as usize {
            let ui_click = XY {
                x: (pointer.pos.x - draw_win_transform.x),
                y: (pointer.pos.y - draw_win_transform.y),
            };
            // println!("{:?}", ui_click);
            let main_tex_click = XY {
                x: ((ui_click.x - self.data.transform.x) as f32 / self.data.screen_zoom) as i32,
                y: ((ui_click.y - self.data.transform.y) as f32 / self.data.screen_zoom) as i32,
            };
            // println!("{:?}", main_tex_click);

            self.tools[self.current_tool].process_stroke(
                &mut self.data,
                main_tex_click,
                pointer,
                canvas,
                textures,
            );
        }
        //draw to buffer
        let ui_tex = textures.open_texture_mut(self.data.targeted_ui_texture);
        // if self.ui_window_was_updated {
        let src_dst = self.calc_canvas_pos(draw_win_transform);
        // this is just a buffer texture to be copied to draw_window directly
        ui_tex.src = Some(src_dst.0);
        ui_tex.dst = Some(src_dst.0);
        self.data
            .history
            .full_draw(canvas, ui_tex, self.data.transform, src_dst.0, src_dst.1);
        // self.ui_window_was_updated = false;
        // }
    }
    pub fn add_zoom(&mut self, zoom_to_add: f32) {
        self.data.screen_zoom += zoom_to_add;
        self.ui_window_was_updated = true;
    }
    fn calc_canvas_pos(&mut self, ui_pos: XYWH) -> (Rect, Rect) {
        let coords = XYWH::new(
            self.data.screen_pos.x,
            self.data.screen_pos.y,
            (self.data.transform.w as f32 * self.data.screen_zoom) as i32,
            (self.data.transform.h as f32 * self.data.screen_zoom) as i32,
        );
        let overlap_w = min(ui_pos.w, coords.x + coords.w) - max(0, coords.x);
        if overlap_w <= 0 {
            return (Rect::new(0, 0, 0, 0), Rect::new(0, 0, 0, 0));
        }
        let overlap_h = min(ui_pos.h, coords.y + coords.h) - max(0, coords.y);
        if overlap_h <= 0 {
            return (Rect::new(0, 0, 0, 0), Rect::new(0, 0, 0, 0));
        }

        let src = Rect::new(
            -min(0, coords.x),
            -min(0, coords.y),
            (overlap_w as f32 / self.data.screen_zoom) as u32,
            (overlap_h as f32 / self.data.screen_zoom) as u32,
        );
        let dst = Rect::new(
            ui_pos.x + max(0, coords.x),
            ui_pos.y + max(0, coords.y),
            (overlap_w) as u32,
            (overlap_h) as u32,
        );
        println!("src: {:?}, dst: {:?}", src, dst,);
        (src, dst)
    }
}
