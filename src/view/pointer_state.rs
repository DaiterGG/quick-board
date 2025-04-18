use super::ui_builder::Id;

pub struct PointerState {
    pub updated: bool,
    pub x: i32,
    pub y: i32,
    // TODO: put in the array
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
    pub interacting_with: Option<Id>,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonState {
    Pressed, // on this frame
    Held,
    Released, // on this frame
    Idle,
}

impl PointerState {
    pub fn new() -> PointerState {
        // NOTE: if pointer is held on launch - bug
        // NOTE: if poiter is not moved on launch - bug
        // FIXME: (optional)
        PointerState {
            x: 0,
            y: 0,
            updated: false,
            left: ButtonState::Idle,
            right: ButtonState::Idle,
            middle: ButtonState::Idle,
            interacting_with: None,
        }
    }
    pub fn reset(&mut self) {
        self.left = match self.left {
            ButtonState::Pressed => ButtonState::Held,
            ButtonState::Released => ButtonState::Idle,
            _ => self.left,
        };
        self.middle = match self.middle {
            ButtonState::Pressed => ButtonState::Held,
            ButtonState::Released => ButtonState::Idle,
            _ => self.middle,
        };
        self.right = match self.right {
            ButtonState::Pressed => ButtonState::Held,
            ButtonState::Released => ButtonState::Idle,
            _ => self.right,
        };
        self.updated = false;
    }
}
