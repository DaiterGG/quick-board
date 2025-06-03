use app::{
    action_pump::*, input_state::ButtonState, texture_data::TextureData, texture_vec::TexId16,
};
use sdl2::{pixels::Color, render::TextureAccess};

use crate::*;

use super::{
    coords::*, history::History, input_state::InputState, predefined::*,
    style_display::DisplayState, texture_manager::*, tool_trait::*, ui_map::UIMap,
};

pub type StepId = usize;
pub struct CanvasManager {
    pub data: CanvasData,
    pub current_tool: ToolId,
    pub tools: Tools,
    previous_tool: Option<ToolId>,
    update_cursor: bool,
}
/// data to pass to tools and save to config
///
/// * `transform`: WH of the canvas, XY texture offset
/// * `screen_pos`: position of the canvas inside draw_window
/// * `screen_zoom`: zoom multiplier
/// * `targeted_ui_texture`: id of the texture that is drawn to draw_window
pub struct CanvasData {
    pub color: Observed<Color>,

    pub last_hue: f32,
    pub last_saturation: f32,

    pub transform: XYWH,
    pub screen_pos: XY,
    pub screen_zoom: f32,
    pub targeted_ui_texture: TexId16,
    pub targeted_ui_element: Id32,
    pub history: History,
}
const BIG_CANVAS: i32 = 10_000;
const DEFAULT_COLOR: Color = Color::RGB(214, 214, 214);
impl CanvasManager {
    pub fn new(t_manager: &mut TextureManager, ui_map: &mut UIMap, window_id: Id32) -> Self {
        let targeted_ui_texture = t_manager.textures.init_texture(TextureData::new(
            &t_manager.t_creator,
            WH::new_one(t_manager.biggest_possible_resolution),
            None,
            Some(TextureAccess::Target),
        ));
        let draw_win_display = ui_map.displays.get_mut_unwrap(window_id).states_data
            [DisplayState::Idle as usize]
            .as_mut()
            .unwrap();
        draw_win_display.set_tex(targeted_ui_texture);
        t_manager.textures.init_palettes();

        ActionPump::add(Action::ColorFullUpdate(DEFAULT_COLOR));
        Self {
            data: CanvasData {
                last_hue: 0.0,
                last_saturation: 0.0,
                color: Observed::new(DEFAULT_COLOR, Box::new(Action::ColorObserve)),
                screen_zoom: 1.0,
                screen_pos: XY::new(BIG_CANVAS / -2, BIG_CANVAS / -2),
                transform: XYWH::new(0, 0, BIG_CANVAS, BIG_CANVAS),
                history: History::new(),
                targeted_ui_texture,
                targeted_ui_element: window_id,
            },
            // ui_window_was_updated: true,
            update_cursor: true,
            current_tool: ToolId::Brush,
            previous_tool: None,
            tools: Tools::init_all_tools(t_manager),
        }
    }
    pub fn update(
        &mut self,
        input: &mut InputState,
        ui_map: &mut UIMap,
        t_manager: &mut TextureManager,
    ) {
        let draw_win_transform = ui_map.elements.get(self.data.targeted_ui_element).transform;

        let stroke_at = input
            .pos
            .substract(draw_win_transform.xy())
            .transform_from(self.data.screen_zoom, self.data.screen_pos);

        match self.current_tool {
            ToolId::Brush => {
                self.tools
                    .brush
                    .process_stroke(&mut self.data, input, stroke_at, t_manager);
            }
            ToolId::Move => {
                self.tools.move_tool.process_stroke(&mut self.data, input);
            }
            ToolId::Sample => {
                self.tools
                    .sample
                    .process_stroke(&mut self.data, input, t_manager);
            }
            _ => {}
        }
        if self.update_cursor {
            input.cursor.update(
                self.data.screen_zoom,
                self.tools.get_size(self.current_tool),
            );

            self.update_cursor = false;
        }
        //draw to buffer
        let ui_tex = &mut t_manager.textures.get_mut(self.data.targeted_ui_texture);
        let dst = self.calc_canvas_pos(draw_win_transform.wh());

        // this is just a buffer texture to be copied to draw_window directly
        ui_tex.src = Some(dst);
        ui_tex.dst = Some(dst);

        self.data.history.full_draw(t_manager, &self.data, dst);
    }

    pub fn change_tool(&mut self, tool_id: ToolId) {
        self.current_tool = tool_id;
        self.update_cursor = true;
        // self.tools.enable(tool_id, &mut self.data);
    }
    pub fn add_zoom(&mut self, zoom_to_add: f32) {
        self.data.screen_zoom += zoom_to_add;
        self.update_cursor = true;
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
            self.change_tool(tool_id);
        } else if !hold_in && self.current_tool == tool_id && self.previous_tool.is_some() {
            self.change_tool(self.previous_tool.unwrap());
            self.previous_tool = None;
        }
    }

    pub fn add_tool_size(&mut self, add: f32, t_manager: &mut TextureManager) {
        self.tools.add_size(self.current_tool, add, t_manager);
        self.update_cursor = true;
    }
    pub fn mult_tool_size(&mut self, up: bool, t_manager: &mut TextureManager) {
        let by = if up { 1.1 } else { 0.9 };
        self.tools.mult_size(self.current_tool, by, t_manager);
        self.update_cursor = true;
    }
}
