use sdl2::{EventPump, Sdl, event::*, keyboard::Keycode, mouse::*};

use crate::{app::input_state::ButtonState, dl};

use super::{action_pump::*, coords::*, input_state::InputState, tool_trait::*, ui_manager::*};

pub struct EventManager {
    pump: EventPump,
    shift_pressed: bool,
    ctrl_pressed: bool,
}
type UserQuit = bool;
impl EventManager {
    pub fn new(sdl: &Sdl) -> Result<EventManager, String> {
        Ok(Self {
            shift_pressed: false,
            ctrl_pressed: false,
            pump: sdl.event_pump()?,
        })
    }
    pub fn handle_events(
        &mut self,
        input: &mut InputState,
        ui: &mut UIManager,
        action_pump: &mut ActionPump,
    ) -> Result<UserQuit, String> {
        // let mouse_state = MouseState::new(&self.pump);

        use Action::*;
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(true),
                Event::Display { display_event, .. } => {
                    // println!("{:?}", display_event);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    ui.window_size = WH { w, h };
                    ui.requires_update = true;
                }

                Event::MouseMotion { x, y, .. } => {
                    input.updated = true;
                    input.delta = XY::new(x - input.pos.x, y - input.pos.y);
                    input.pos = XY::new(x, y);
                }
                Event::MouseWheel { y, .. } => {
                    input.updated = true;
                    input.scroll_y = y;
                }
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Space) => {
                        action_pump.add(HoldTool(ToolId::Move, false));
                    }
                    Some(Keycode::LShift) => input.shift.0 = false,
                    Some(Keycode::RShift) => input.shift.1 = false,
                    Some(Keycode::LCtrl) => input.ctrl.0 = false,
                    Some(Keycode::RCtrl) => input.ctrl.1 = false,
                    Some(Keycode::LAlt) => input.alt.0 = false,
                    Some(Keycode::RAlt) => input.alt.1 = false,
                    _ => {}
                },
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Space) => {
                        action_pump.add(HoldTool(ToolId::Move, true));
                    }
                    Some(Keycode::LShift) => input.shift.0 = true,
                    Some(Keycode::RShift) => input.shift.1 = true,
                    Some(Keycode::LCtrl) => input.ctrl.0 = true,
                    Some(Keycode::RCtrl) => input.ctrl.1 = true,
                    Some(Keycode::LAlt) => input.alt.0 = true,
                    Some(Keycode::RAlt) => input.alt.1 = true,
                    Some(Keycode::F) => {
                        action_pump.add(BrushSize(true));
                    }
                    Some(Keycode::A) => {
                        action_pump.add(BrushSize(false));
                    }
                    Some(Keycode::Z) if input.ctrl() && !input.shift() => {
                        action_pump.add(Undo);
                    }
                    Some(Keycode::Z) if input.shift() && !input.ctrl() => {
                        action_pump.add(Redo);
                    }
                    _ => {}
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    input.updated = true;
                    input.states[mouse_btn as usize] = ButtonState::Pressed;
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    input.updated = true;
                    input.states[mouse_btn as usize] = ButtonState::Released;
                }
                _ => {}
            }
        }
        Ok(false)
    }
}
