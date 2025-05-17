use sdl2::mouse::MouseButton;

use crate::{d, dl};

use super::{coords::XY, predefined::*};

pub struct InputState {
    pub updated: bool,
    pub pos: XY,
    pub delta: XY,
    pub states: [ButtonState; 6],
    pub interacting_with: Option<IdI32>,
    pub shift: (bool, bool),
    pub ctrl: (bool, bool),
    pub alt: (bool, bool),
    pub start_holding_at: Option<XY>,
    pub scroll_y: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonState {
    Pressed, // on this frame
    Held,
    Released, // on this frame
    Idle,
}

impl InputState {
    pub fn new() -> InputState {
        // NOTE: if pointer is held on launch - U/B
        // NOTE: if poiter is not moved on launch - U/B
        // FIXME: (optional)
        InputState {
            pos: XY::new(0, 0),
            updated: false,
            delta: XY::new(0, 0),
            states: [ButtonState::Idle; 6],
            start_holding_at: None,
            interacting_with: None,
            shift: (false, false),
            ctrl: (false, false),
            alt: (false, false),
            scroll_y: 0,
        }
    }
    pub fn left(&self) -> ButtonState {
        self.states[MouseButton::Left as usize]
    }
    pub fn shift(&self) -> bool {
        self.shift.0 || self.shift.1
    }
    pub fn ctrl(&self) -> bool {
        self.ctrl.0 || self.ctrl.1
    }
    pub fn alt(&self) -> bool {
        self.alt.0 || self.alt.1
    }
    pub fn reset(&mut self) {
        use ButtonState::*;

        for state in self.states.iter_mut() {
            *state = match *state {
                Pressed => Held,
                Released => Idle,
                _ => *state,
            }
        }
        use MouseButton::*;
        if self.states[Left as usize] == Idle
            && self.states[Middle as usize] == Idle
            && self.states[Right as usize] == Idle
        {
            self.interacting_with = None;
        }
        self.scroll_y = 0;
        self.updated = false;
        self.delta = XY::new(0, 0);
    }
    pub fn mult(&self) -> f32 {
        if self.ctrl() {
            0.2
        } else if self.shift() {
            5.0
        } else if self.alt() {
            15.0
        } else {
            1.0
        }
    }
}
