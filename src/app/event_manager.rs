use sdl2::{EventPump, Sdl, event::*, keyboard::Keycode, mouse::*};

use crate::dl;

use super::{action_pump::*, coords::*, pointer_state::*, tool_trait::ToolId, ui_manager::*};

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
        pointer: &mut PointerState,
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
                    pointer.updated = true;
                    pointer.pos = XY::new(x, y);
                }
                Event::MouseWheel { y, .. } => {
                    pointer.updated = true;
                    pointer.scroll_y = y;
                }
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Space) => {
                        action_pump.add(HoldTool(ToolId::Move, false));
                    }
                    Some(Keycode::LShift) => self.shift_pressed = false,
                    Some(Keycode::LCtrl) => self.ctrl_pressed = false,
                    _ => {}
                },
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Space) => {
                        action_pump.add(HoldTool(ToolId::Move, true));
                    }
                    Some(Keycode::LShift) => self.shift_pressed = true,
                    Some(Keycode::LCtrl) => self.ctrl_pressed = true,
                    Some(Keycode::F) => {
                        action_pump.add(BrushSize(true));
                    }
                    Some(Keycode::A) => {
                        action_pump.add(BrushSize(false));
                    }
                    Some(Keycode::Z) if self.ctrl_pressed && !self.shift_pressed => {
                        dl!("Z ctrl");
                        action_pump.add(Undo);
                    }
                    Some(Keycode::Z) if self.ctrl_pressed && self.shift_pressed => {
                        dl!("Z");
                        action_pump.add(Redo);
                    }
                    _ => {}
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    pointer.updated = true;
                    match mouse_btn {
                        MouseButton::Left => pointer.left = ButtonState::Pressed,
                        MouseButton::Right => pointer.right = ButtonState::Pressed,
                        MouseButton::Middle => pointer.middle = ButtonState::Pressed,
                        _ => {}
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    pointer.updated = true;
                    match mouse_btn {
                        MouseButton::Left => pointer.left = ButtonState::Released,
                        MouseButton::Right => pointer.right = ButtonState::Released,
                        MouseButton::Middle => pointer.middle = ButtonState::Released,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }
}
