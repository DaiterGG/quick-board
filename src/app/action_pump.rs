use super::coords::XY;
use super::pointer_state::PointerState;
use super::predefined::*;

use super::canvas_manager::CanvasManager;
use super::tool_trait::ToolId;
use std::mem::replace;

pub enum Action {
    ButtonPressed(IdI32),
    HoldTool(ToolId, bool),
    Undo,
    Redo,
    BrushSize(bool),
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
    pub fn get_and_clear(&mut self) -> Vec<Action> {
        replace(&mut self.actions, Vec::new())
    }
    pub fn apply(&mut self, canvas_manager: &mut CanvasManager, pointer: &PointerState) {
        let actions = self.get_and_clear();
        use Action::*;
        for action in actions {
            match action {
                ButtonPressed(id) if id == Id::BrushButton as i32 => {
                    canvas_manager.change_tool(ToolId::Brush);
                }
                ButtonPressed(id) if id == Id::MoveButton as i32 => {
                    canvas_manager.change_tool(ToolId::Move);
                }
                HoldTool(id, hold_in) => canvas_manager.try_hold_tool(id, hold_in),
                Undo => canvas_manager.undo(),
                Redo => canvas_manager.redo(),
                BrushSize(increase) => canvas_manager.tools.brush.change_brush_size(increase),
                _ => (),
            }
        }
    }
}
