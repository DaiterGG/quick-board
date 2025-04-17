pub struct PointerState {
    pub updated: bool,
    pub x: i32,
    pub y: i32,
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
}
#[derive(Copy, Clone)]
pub enum ButtonState {
    Pressed,
    Held,
    Released,
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
            left: ButtonState::Released,
            right: ButtonState::Released,
            middle: ButtonState::Released,
        }
    }
    pub fn reset(&mut self) {
        self.left = match self.left {
            ButtonState::Pressed => ButtonState::Held,
            _ => self.left,
        };
        self.middle = match self.middle {
            ButtonState::Pressed => ButtonState::Held,
            _ => self.middle,
        };
        self.right = match self.right {
            ButtonState::Pressed => ButtonState::Held,
            _ => self.right,
        };
        self.updated = false;
    }
}
