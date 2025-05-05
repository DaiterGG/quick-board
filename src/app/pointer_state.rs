use super::{coords::XY, predefined::IdUsize};

pub struct PointerState {
    pub updated: bool,
    pub pos: XY,
    // TODO: put in the array
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
    pub interacting_with: Option<IdUsize>,
    pub scroll_y: i32,
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
            pos: XY::new(0, 0),
            updated: false,
            left: ButtonState::Idle,
            right: ButtonState::Idle,
            middle: ButtonState::Idle,
            interacting_with: None,
            scroll_y: 0,
        }
    }
    pub fn reset(&mut self) {
        use ButtonState::*;
        self.left = match self.left {
            Pressed => Held,
            Released => Idle,
            _ => self.left,
        };
        self.middle = match self.middle {
            Pressed => Held,
            Released => Idle,
            _ => self.middle,
        };
        self.right = match self.right {
            Pressed => Held,
            Released => Idle,
            _ => self.right,
        };
        if self.left == Idle && self.right == Idle && self.middle == Idle {
            self.interacting_with = None;
        }
        self.scroll_y = 0;
        self.updated = false;
    }
}
