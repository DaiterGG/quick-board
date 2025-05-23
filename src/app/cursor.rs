use sdl2::{
    gfx::primitives::DrawRenderer,
    mouse::*,
    pixels::{Color, PixelFormatEnum},
    render::BlendMode,
    surface::Surface,
};

use crate::dl;

use super::coords::WH;

pub struct CursorManager {
    pub canvas_cursor_active: bool,
    pub biggest_possible_resolution: i32,

    // created mouse::Cursor neet to be stored after .set()
    canvas_cursor: Cursor,
}
impl CursorManager {
    pub fn new(current_zoom: f32, current_diameter: i32, bpr: i32) -> Self {
        Self {
            canvas_cursor_active: false,
            biggest_possible_resolution: bpr,
            canvas_cursor: Self::generate(current_zoom, current_diameter, bpr),
        }
    }
    pub fn set_active(&mut self, active: bool) {
        self.canvas_cursor_active = active;
        if active {
            self.canvas_cursor.set();
        } else {
            let cur = Cursor::from_system(SystemCursor::Arrow).unwrap();
            cur.set();
        }
    }
    pub fn update(&mut self, current_zoom: f32, current_diameter: i32) {
        let cur = Self::generate(
            current_zoom,
            current_diameter,
            self.biggest_possible_resolution,
        );
        if self.canvas_cursor_active {
            cur.set();
        }
        self.canvas_cursor = cur;
    }
    fn generate(current_zoom: f32, current_diameter: i32, bpr: i32) -> Cursor {
        let mut r = (current_zoom * current_diameter as f32) as i16 / 2;
        if r < 1 || r > bpr as i16 || r > 1000 {
            r = 1;
        }

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
        Cursor::from_surface(surface, r as i32, r as i32).unwrap()
    }
}
