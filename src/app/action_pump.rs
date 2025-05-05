use super::coords::XY;
use super::pointer_state::PointerState;
use super::predefined::{Id, IdUsize};

use super::canvas_manager::CanvasManager;
use super::tool_trait::ToolId;
use std::mem::replace;

pub enum Action {
    ButtonPressed(IdUsize),
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
        for action in actions {
            match action {
                Action::ButtonPressed(id) if id == Id::BrushButton as usize => {
                    canvas_manager.change_tool(ToolId::Brush);
                }
                Action::ButtonPressed(id) if id == Id::MoveButton as usize => {
                    canvas_manager.change_tool(ToolId::Move);
                }
                // Action::CanvasPressed(pos) => {
                //     //
                // }
                _ => (),
            }
        }
    }
}
