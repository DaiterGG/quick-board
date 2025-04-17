use sdl2::{
    EventPump, Sdl,
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
};

use super::{
    coords::{WH, XYWH},
    pointer_state::ButtonState,
    states::States,
    ui_manager::{self, UIManager},
};

pub struct EventManager {
    pump: EventPump,
}
impl EventManager {
    pub fn new(sdl: &Sdl) -> Result<EventManager, String> {
        Ok(Self {
            pump: sdl.event_pump()?,
        })
    }
    pub fn handle_events(&mut self, states: &mut States) -> Result<(), String> {
        // let mouse_state = MouseState::new(&self.pump);
        let pntr = &mut states.pointer;
        let ui = &mut states.ui;

        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Err("USER QUIT".to_owned()),
                Event::Display { display_event, .. } => {
                    println!("{:?}", display_event);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    ui.window_size = WH { w, h };
                    ui.requires_update = true;
                }

                Event::MouseMotion { x, y, .. } => {
                    pntr.x = x;
                    pntr.y = y;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    pntr.updated = true;
                    match mouse_btn {
                        MouseButton::Left => pntr.left = ButtonState::Pressed,
                        MouseButton::Right => pntr.right = ButtonState::Pressed,
                        MouseButton::Middle => pntr.middle = ButtonState::Pressed,
                        _ => {}
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Left => pntr.left = ButtonState::Released,
                    MouseButton::Right => pntr.right = ButtonState::Released,
                    MouseButton::Middle => pntr.middle = ButtonState::Released,
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }
}
