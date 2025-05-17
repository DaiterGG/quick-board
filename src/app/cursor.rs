use sdl2::{
    gfx::primitives::DrawRenderer,
    mouse::*,
    pixels::{Color, PixelFormatEnum},
    surface::Surface,
};

pub struct CanvasCursor {
    pub active: bool,

    // created mouse::Cursor neet to be stored after .set()
    data: Cursor,
}
impl CanvasCursor {
    pub fn new() -> Self {
        Self {
            active: false,
            data: Cursor::from_system(SystemCursor::Arrow).unwrap(),
        }
    }
    pub fn update(&mut self, current_zoom: f32, current_diameter: i32) {
        let r = (current_zoom * current_diameter as f32) as i16 / 2;
        let surface = Surface::new(
            r as u32 * 2 + 1,
            r as u32 * 2 + 1,
            PixelFormatEnum::RGBA8888,
        )
        .unwrap();

        let can = surface.into_canvas().unwrap();
        can.circle(r, r, r, Color::RGBA(180, 180, 180, 255))
            .unwrap();
        let surface = can.into_surface();

        // takes 40 ms for 1400 x 1400
        let new = Cursor::from_surface(surface, r as i32, r as i32).unwrap();

        new.set();

        self.data = new;
    }
}
