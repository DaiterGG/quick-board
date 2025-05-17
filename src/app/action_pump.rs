use crate::dl;

use super::coords::{XY, XYF32};
use super::cursor::CanvasCursor;
use super::input_state::InputState;
use super::predefined::*;

use super::canvas_manager::CanvasManager;
use super::texture_manager::TextureManager;
use super::tool_trait::ToolId;
use super::ui_manager::UIManager;
use std::mem::take;

pub enum Action {
    ButtonPressed(IdI32),
    HoldTool(ToolId, bool),
    Undo,
    Redo,
    Drag(IdI32, XYF32),
    BrushSize(bool),
    CursorInCanvas(bool),
}

pub struct ActionPump {
    pub actions: Vec<Action>,
}
impl ActionPump {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }
    pub fn apply(
        &mut self,
        c_manager: &mut CanvasManager,
        ui: &mut UIManager,
        input: &InputState,
        t_manager: &mut TextureManager,
    ) {
        let actions = take(&mut self.actions);
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
                    if c_manager.current_tool == ToolId::Brush {
                        c_manager.add_brush_size(d_x, t_manager);
                    } else if c_manager.current_tool == ToolId::Move {
                        //
                    }
                }
                HoldTool(id, hold_in) => c_manager.try_hold_tool(id, hold_in),
                Undo => c_manager.undo(t_manager),
                Redo => c_manager.redo(),
                BrushSize(increase) => {
                    c_manager.mult_brush_size(increase, t_manager);
                }
                CursorInCanvas(in_canvas) => {
                    c_manager.cursor.active = in_canvas;
                }

                _ => (),
            }
        }
    }
}
