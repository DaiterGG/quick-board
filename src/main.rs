extern crate sdl2;

mod model;
mod relay;
mod view;

use crate::view::{config::Config, ui_manager::UIManager};
use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    mouse::{MouseButton, MouseState},
    pixels::Color,
    render::{BlendMode, Canvas, CanvasBuilder, TextureCreator},
    video::{Window, WindowContext},
};
use std::time::Duration;
use view::coords::XYWH;

const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 100;
const COLOR: Color = Color::RGB(200, 200, 200);
const RADIUS: i16 = 3;

// TODO:'s
// event file
// event call ui_manager.pointer_moved() recursevly
// event call cfg.update_window_size()
// rename ctx to cfg
//

pub fn main() -> Result<(), String> {
    let mut cfg = Config::init()?;

    let t_creator: TextureCreator<WindowContext> = cfg.canvas.texture_creator();

    let mut ui_manager = UIManager::new();
    let ws = cfg.window_size();
    ui_manager.update_pos(XYWH::new(0, 0, ws.w, ws.h), &cfg);

    // TODO: remove
    let mut draw_texture = t_creator
        .create_texture_target(
            t_creator.default_pixel_format(),
            SCREEN_WIDTH * 2,
            SCREEN_HEIGHT * 2,
        )
        .unwrap();
    draw_texture.set_blend_mode(BlendMode::Blend);

    'main_loop: loop {
        let canvas = &mut cfg.canvas;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));

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
                } => break 'main_loop,
                Event::Display { display_event, .. } => {
                    println!("{:?}", display_event);
                }

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    // println!("Mouse button down at {} {}", x, y);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&draw_texture, None, None).unwrap();
        ui_manager.draw_to(canvas);
        canvas.circle(x, y, 20, Color::RGB(255, 0, 0)).unwrap();
        canvas.present();
    }
    Ok(())
}
