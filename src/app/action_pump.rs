use crate::app::canvas_manager;
use crate::app::color_operations::ColorOperations;
use crate::dl;

use super::coords::{XY, XYF32};
use super::cursor::CursorManager;
use super::input_state::InputState;
use super::predefined::*;

use super::canvas_manager::CanvasManager;
use super::texture_manager::TextureManager;
use super::tool_trait::ToolId;
use super::ui_manager::UIManager;
use std::mem::take;
use std::sync::{Mutex, OnceLock};

#[derive(Copy, Clone)]
pub enum Action {
    ButtonPressed(IdI32),
    HoldTool(ToolId, bool),
    Undo,
    Redo,
    Drag(IdI32, XYF32),
    SliderLine(IdI32, f32),
    SladerCoord(IdI32, XYF32),
    BrushSize(bool),
    CursorInCanvas(bool),
    UIUpdate,
    WindowResized,

    //observers
    ColorChanged,
}

pub static A_PUMP: OnceLock<ActionPump> = OnceLock::new();

pub struct ActionPump {
    pub actions: Mutex<Vec<Action>>,
}
impl ActionPump {
    pub fn init() {
        A_PUMP.set(ActionPump {
            actions: Mutex::new(Vec::new()),
        });
    }

    pub fn add(action: Action) {
        A_PUMP.get().unwrap().actions.lock().unwrap().push(action);
    }
    pub fn apply(
        c_manager: &mut CanvasManager,
        ui: &mut UIManager,
        input: &mut InputState,
        t_manager: &mut TextureManager,
    ) {
        let this = A_PUMP.get().unwrap();
        let actions = take(&mut (*this.actions.lock().unwrap()));
        // let actions = take(&mut self.actions.lock().unwrap());
        use Action::*;
        for action in actions {
            match action {
                ButtonPressed(id) if id == Id::BrushButton as i32 => {
                    c_manager.change_tool(ToolId::Brush);
                }
                ButtonPressed(id) if id == Id::MoveButton as i32 => {
                    c_manager.change_tool(ToolId::Move);
                }
                Drag(id, delta) if id == Id::ToolSizeDrag as i32 && delta.x != 0.0 => {
                    const SIZE_DRAG_MULT: f32 = 0.5;
                    let d_x = delta.x * SIZE_DRAG_MULT * ui.ui_scale;
                    c_manager.add_tool_size(d_x, t_manager);
                    input.cursor.set_active(true);
                }
                HoldTool(id, hold_in) => c_manager.try_hold_tool(id, hold_in),
                Undo => c_manager.data.history.undo(t_manager),
                Redo => c_manager.data.history.redo(),
                BrushSize(increase) => {
                    c_manager.mult_tool_size(increase, t_manager);
                }
                CursorInCanvas(in_canvas) => {
                    input.cursor.set_active(in_canvas);
                }
                SliderLine(id, ratio) if id == Id::HSV_H as i32 => {
                    let color = ColorOperations::apply_hue(c_manager.data.color.get(), ratio);
                    c_manager.data.color.set(color);
                }
                SliderLine(id, ratio) if id == Id::HSV_S as i32 => {
                    let color =
                        ColorOperations::apply_saturation(c_manager.data.color.get(), ratio);
                    c_manager.data.color.set(color);
                }
                SliderLine(id, ratio) if id == Id::HSV_V as i32 => {
                    let color = ColorOperations::apply_value(c_manager.data.color.get(), ratio);
                    c_manager.data.color.set(color);
                }
                UIUpdate => {
                    ui.requires_update = true;
                }
                WindowResized => {
                    c_manager.tools.brush.generate_circle_mask(t_manager);
                    c_manager.tools.brush.update_buffer(t_manager);
                    ui.requires_update = true;
                }

                ColorChanged => {
                    t_manager.update_palettes(c_manager.data.color.get());
                }
                _ => (),
            }
        }
    }
}
pub struct Observed<T> {
    value: T,
    action: Action,
}
impl<T: Copy> Observed<T> {
    pub fn get(&self) -> T {
        self.value
    }
}
impl<T> Observed<T> {
    pub fn new(value: T, action: Action) -> Observed<T> {
        ActionPump::add(action);
        Observed { value, action }
    }
    // pub fn get_ref(&self) -> &T {
    //     &self.value
    // }
    pub fn set(&mut self, value: T) {
        self.value = value;
        ActionPump::add(self.action);
    }
}
