use app::texture_data::TextureData;
use sdl2::{pixels::Color, rect::Rect, render::*, video::Window};

use crate::*;

use super::{
    coords::*,
    history::History,
    pointer_state::PointerState,
    predefined::{Id, IdI32},
    style_display::DisplayState,
    texture_manager::*,
    tool_trait::*,
    ui_map::UIMap,
};

pub type StepId = usize;
pub struct CanvasManager {
    pub data: CanvasData,
    // ui_window_was_updated: bool,
    pub current_tool: ToolId,
    pub tools: Tools,
    previous_tool: Option<ToolId>,
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
    pub targeted_ui_element: IdI32,
    pub history: History,
}
const BIG_CANVAS: i32 = 10_000;
impl CanvasManager {
    pub fn new(t_manager: &mut TextureManager, ui_map: &mut UIMap, window_id: IdI32) -> Self {
        let targeted_ui_texture = t_manager.init_open_texture(TextureData::new(
            &t_manager.t_creator,
            t_manager.biggest_possible_resolution,
            None,
        ));
        let draw_win_display = ui_map.displays[window_id as usize]
            .as_mut()
            .unwrap()
            .states_data[DisplayState::Idle as usize]
            .as_mut()
            .unwrap();
        *draw_win_display = draw_win_display.open_texture(targeted_ui_texture);
        Self {
            data: CanvasData {
                // screen_pos: XY::new(100, 100),
                // transform: XYWH::new(0, 0, 400, 300),
                // screen_zoom: 1.0,
                screen_zoom: 1.0,
                screen_pos: XY::new(BIG_CANVAS / -2, BIG_CANVAS / -2),
                transform: XYWH::new(0, 0, BIG_CANVAS, BIG_CANVAS),
                history: History::new(),
                targeted_ui_texture,
                targeted_ui_element: window_id,
            },
            // ui_window_was_updated: true,
            current_tool: ToolId::Brush,
            previous_tool: None,
            tools: Tools::init_all_tools(t_manager),
        }
    }
    pub fn update(
        &mut self,
        pointer: &PointerState,
        ui_map: &mut UIMap,
        textures: &mut TextureManager,
    ) {
        // d!(self.data.screen_pos);
        // d!(self.data.screen_zoom);
        // dl!(self.data.transform);
        let draw_win_transform = ui_map.elements[self.data.targeted_ui_element as usize].transform;

        match self.current_tool {
            ToolId::Brush => {
                let stroke_at = pointer
                    .pos
                    .substract(draw_win_transform.xy())
                    .transform_from(self.data.screen_zoom, self.data.screen_pos);
                self.tools
                    .brush
                    .process_stroke(&mut self.data, pointer, stroke_at, textures);
            }
            ToolId::Move => {
                self.tools.move_tool.process_stroke(&mut self.data, pointer);
            }
            _ => {}
        }

        //draw to buffer
        let ui_tex_id = self.data.targeted_ui_texture;
        let ui_tex = &mut textures.open_textures[ui_tex_id];
        let dst = self.calc_canvas_pos(draw_win_transform.wh());

        // this is just a buffer texture to be copied to draw_window directly
        ui_tex.src = Some(dst);
        ui_tex.dst = Some(dst);

        let _ = textures
            .canvas
            .with_texture_canvas(&mut ui_tex.texture, |c| {
                c.set_draw_color(Color::RGB(20, 20, 20));
                let _ = c.fill_rect(dst.to_rect());
            });
        self.data.history.full_draw(textures, &self.data, dst);
    }
    pub fn change_tool(&mut self, tool_id: ToolId) {
        self.current_tool = tool_id;
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
    fn calc_canvas_pos(&mut self, ui_size: WH) -> XYWH {
        let to_ui_space = self
            .data
            .transform
            .transform_into(self.data.screen_zoom, self.data.screen_pos);

        to_ui_space.get_overlap(ui_size)
    }
    pub fn try_hold_tool(&mut self, tool_id: ToolId, hold_in: bool) {
        if hold_in && self.current_tool != tool_id && self.previous_tool.is_none() {
            self.previous_tool = Some(self.current_tool);
            self.current_tool = tool_id;
        } else if !hold_in && self.current_tool == tool_id && self.previous_tool.is_some() {
            self.current_tool = self.previous_tool.unwrap();
            self.previous_tool = None;
        }
    }

    pub fn undo(&mut self) {
        dl!(self.data.history.selected_h_step);
        if let Some(id) = self.data.history.selected_h_step {
            if id > 0 {
                self.data.history.selected_h_step = Some(id - 1);
            } else {
                self.data.history.selected_h_step = None;
            }
        }
    }
    pub fn redo(&mut self) {
        if let Some(id) = self.data.history.selected_h_step {
            if id < self.data.history.steps.len() - 1 {
                self.data.history.selected_h_step = Some(id + 1);
            }
        } else {
            self.data.history.selected_h_step = Some(0);
        }
    }
}
