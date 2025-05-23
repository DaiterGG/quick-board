use sdl2::{EventPump, Sdl, event::*, keyboard::Keycode, mouse::*};

use crate::{
    TextureManager,
    app::{canvas_manager, input_state::ButtonState, texture_manager},
    d, dl,
};

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
        texture_manager: &mut TextureManager,
        sdl: &Sdl,
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
                    // potentially better way to handle it:
                    // https://github.com/Rust-SDL2/rust-sdl2/issues/1243#issuecomment-2657420674
                    ActionPump::add(WindowResized);
                }

                Event::MouseMotion { x, y, .. } => {
                    input.delta = XY::new(x - input.pos.x, y - input.pos.y);
                    input.pos = XY::new(x, y);
                    if input.mouse_wrap_on {
                        let w_size = texture_manager.canvas.window().size();
                        if x < 5 && input.delta.x < 0 {
                            input.pos.x = w_size.0 as i32 - 10;
                            sdl.mouse().warp_mouse_in_window(
                                texture_manager.canvas.window(),
                                input.pos.x,
                                input.pos.y,
                            );
                        } else if x > w_size.0 as i32 - 5 && input.delta.x > 0 {
                            input.pos.x = 10;
                            sdl.mouse().warp_mouse_in_window(
                                texture_manager.canvas.window(),
                                input.pos.x,
                                input.pos.y,
                            );
                        }
                        if y < 5 && input.delta.y < 0 {
                            input.pos.y = w_size.1 as i32 - 10;
                            sdl.mouse().warp_mouse_in_window(
                                texture_manager.canvas.window(),
                                input.pos.x,
                                input.pos.y,
                            );
                        } else if y > w_size.1 as i32 - 5 && input.delta.y > 0 {
                            input.pos.y = 10;
                            sdl.mouse().warp_mouse_in_window(
                                texture_manager.canvas.window(),
                                input.pos.x,
                                input.pos.y,
                            );
                        }
                    }
                    input.updated = true;
                }
                Event::MouseWheel { y, .. } => {
                    input.updated = true;
                    input.scroll_y = y;
                }
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Space) => {
                        ActionPump::add(HoldTool(ToolId::Move, false));
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
                        ActionPump::add(HoldTool(ToolId::Move, true));
                    }
                    Some(Keycode::LShift) => input.shift.0 = true,
                    Some(Keycode::RShift) => input.shift.1 = true,
                    Some(Keycode::LCtrl) => input.ctrl.0 = true,
                    Some(Keycode::RCtrl) => input.ctrl.1 = true,
                    Some(Keycode::LAlt) => input.alt.0 = true,
                    Some(Keycode::RAlt) => input.alt.1 = true,
                    Some(Keycode::F) => {
                        ActionPump::add(BrushSize(true));
                    }
                    Some(Keycode::A) => {
                        ActionPump::add(BrushSize(false));
                    }
                    Some(Keycode::Z) if input.ctrl() && !input.shift() => {
                        ActionPump::add(Undo);
                    }
                    Some(Keycode::Z) if input.shift() && !input.ctrl() => {
                        ActionPump::add(Redo);
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
