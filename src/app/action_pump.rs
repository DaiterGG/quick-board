use super::coords::XY;
use super::pointer_state::PointerState;
use super::predefined::{Id, IdUsize};

use super::canvas_manager::CanvasManager;
use super::texture_manager;
use std::fs::canonicalize;
use std::mem::replace;

pub enum Action {
    ButtonPressed(IdUsize),
    CanvasPressed(XY),
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
                Action::ButtonPressed(id) if id == Id::BrushButton1 as usize => {
                    // canvas_manager.screen_pos = XY {
                    //     x: canvas_manager.screen_pos.x + 10,
                    //     y: canvas_manager.screen_pos.y + 10,
                    // };
                    canvas_manager.add_zoom(0.05);
                }
                Action::ButtonPressed(id) if id == Id::BrushButton2 as usize => {
                    canvas_manager.add_zoom(-0.05);
                }
                Action::CanvasPressed(pos) => {
                    //
                }
                _ => (),
            }
        }
    }
}
