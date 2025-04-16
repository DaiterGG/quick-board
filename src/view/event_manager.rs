use sdl2::{EventPump, Sdl};

pub struct EventManager {
    pump: EventPump,
}
impl EventManager {
    pub fn new(sdl: &Sdl) -> Result<EventManager, String> {
        Ok(Self {
            pump: sdl.event_pump()?,
        })
    }
    pub fn handle_events(&mut self) {}
}
