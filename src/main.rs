extern crate sdl2;

mod div;
mod style;
mod transform;

use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
    pixels::Color,
    render::CanvasBuilder,
};
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const COLOR: Color = Color::RGB(0, 255, 255);
const RADIUS: i16 = 10;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("foo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;

    let t_creator = canvas.texture_creator();
    let mut draw_texture = t_creator
        .create_texture_target(
            t_creator.default_pixel_format(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .unwrap();
    // let color = Color::RGB(0, 255, 255);

    'running: loop {
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let mouse_state = MouseState::new(&event_pump);
        let x = MouseState::x(&mouse_state) as i16;
        let y = MouseState::y(&mouse_state) as i16;

        if mouse_state.is_mouse_button_pressed(MouseButton::Left) {
            let _ = canvas.with_texture_canvas(&mut draw_texture, |c| {
                c.filled_circle(x, y, RADIUS, COLOR).unwrap();
            });
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    // println!("Mouse button down at {} {}", x, y);
                }
                _ => {}
            }
        }

        canvas.copy(&draw_texture, None, None).unwrap();
        canvas.circle(x, y, 20, Color::RGB(255, 0, 0)).unwrap();
        canvas.present();
    }
    Ok(())
}
