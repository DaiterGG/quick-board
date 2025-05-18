#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate sdl2;

use std::{env, time::Duration};

use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::*,
    surface::Surface,
    sys::{
        SDL_BlendFactor::*, SDL_BlendMode, SDL_BlendOperation::*, SDL_ComposeCustomBlendMode,
        SDL_SetTextureBlendMode,
    },
    video::*,
};

const RAD: i16 = 600;
const RAD_U32: u32 = RAD as u32;
const COLOR: Color = Color::RGB(255, 0, 0);
pub fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;
    let video_subsystem = sdl.video()?;

    let mut window = video_subsystem
        .window("Quick Board", 600, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let mut alfa_mask = t_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, RAD_U32, RAD_U32)
        .unwrap();
    let mut color_mask = t_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, RAD_U32, RAD_U32)
        .unwrap();
    let mut texture_res = t_creator
        .create_texture_target(PixelFormatEnum::RGBA8888, RAD_U32, RAD_U32)
        .unwrap();

    canvas.with_texture_canvas(&mut alfa_mask, |c| {
        c.set_draw_color(Color::RGBA(0, 0, 0, 0));
        c.clear();
        c.filled_circle(RAD / 2, RAD / 2, RAD / 2, Color::RGBA(255, 255, 255, 128));
        c.filled_circle(RAD / 2, RAD / 2, RAD / 4, Color::RGBA(255, 255, 255, 255));
    });
    let bl_mode: SDL_BlendMode;
    unsafe {
        bl_mode = SDL_ComposeCustomBlendMode(
            SDL_BLENDFACTOR_ZERO,
            SDL_BLENDFACTOR_DST_COLOR,
            SDL_BLENDOPERATION_ADD,
            SDL_BLENDFACTOR_ZERO,
            SDL_BLENDFACTOR_SRC_ALPHA,
            SDL_BLENDOPERATION_ADD,
        );
        SDL_SetTextureBlendMode(alfa_mask.raw(), bl_mode);
    }

    canvas.with_texture_canvas(&mut color_mask, |c| {
        // c.set_draw_color(Color::RGBA(255, 0, 0, 255));
        // c.clear();
        // c.copy(&texture_mask, None, None);
        c.set_draw_color(Color::RGBA(0, 0, 0, 0));
        c.clear();
        c.filled_circle(RAD / 2, RAD / 2, RAD / 2, Color::RGBA(255, 255, 255, 128));
        c.filled_circle(RAD / 2, RAD / 2, RAD / 4, Color::RGBA(255, 255, 255, 255));
        // c.set_draw_color(Color::RGBA(0, 0, 0, 0));
        // c.clear();
        // c.set_draw_color(Color::RGBA(255, 0, 0, 255));
        // c.fill_rect(Rect::new(0, 0, RAD as u32 / 4, RAD as u32));
        // c.set_draw_color(Color::RGBA(255, 0, 0, 128));
        // c.fill_rect(Rect::new(0, 0, RAD as u32 / 2, RAD as u32));
    });

    color_mask.set_color_mod(128, 0, 0);
    color_mask.set_blend_mode(BlendMode::Blend);
    canvas.with_texture_canvas(&mut texture_res, |c| {
        c.set_draw_color(Color::RGB(0, 50, 50));
        c.clear();
        c.filled_circle(RAD / 4, RAD / 4, RAD / 4, Color::RGB(0, 255, 0));
        c.copy(&color_mask, None, None);
    });

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => (),
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture_res, None, None);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // canvas.window_mut().set_size(1000, 700);
    }
    Ok(())
}
