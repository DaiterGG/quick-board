#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate sdl2;

use std::{env, time::Duration};

use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, rect::Rect,
    render::*, surface::Surface, video::*,
};

const RAD: i16 = 4000;
const RAD_U32: u32 = RAD as u32;
pub fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;
    let video_subsystem = sdl.video()?;

    let mut window = video_subsystem
        .window("Quick Board", 800, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let mut texture = t_creator
        .create_texture_target(t_creator.default_pixel_format(), RAD_U32 * 2, RAD_U32 * 2)
        .unwrap();

    canvas.with_texture_canvas(&mut texture, |c| {
        c.filled_pie(RAD, RAD, RAD, -1, 359, Color::RGB(200, 200, 0));
    });

    // target to static
    let mut static_t = t_creator
        .create_texture_static(t_creator.default_pixel_format(), 256, 256)
        .map_err(|e| e.to_string())?;

    canvas.with_texture_canvas(&mut texture, |c| {
        static_t = c
            .create_texture_static(t_creator.default_pixel_format(), 256, 256)
            .unwrap();
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
        canvas.copy(&texture, None, None);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // canvas.window_mut().set_size(1000, 700);
    }
    Ok(())
}

// let mut pointer = PointerState::new();
// let mut ui_manager = UIManager::new(window_size);
// let mut actions = ActionPump::new();
// // TODO: move video_subsystem to texture manager
// let mut texture_manager = TextureManager::new(t_creator, &video_subsystem);
// let mut ui_map = UIMap::new();
// let mut canvas_manager =
//     CanvasManager::new(&mut texture_manager, &mut ui_map, Id::DrawWindow as usize);

// // canvas.with_texture_canvas(
// //     &mut texture_manager
// //         .predefined_mut(TextureId::DrawCanvas)
// //         .texture,
// //     |c| {
// //         c.set_draw_color(Color::RGB(255, 255, 255));
// //         c.clear();
// //     },
// // );
// // let mut fps = FPSManager::new();
// // println!("err {:?}", fps.set_framerate(200));

// let mut time = std::time::Instant::now();
// let mut last_frame = std::time::Instant::now();
// let mut lazy_buffer = std::time::Instant::now();
// let mut frames = 0;
// 'main: loop {
//     // Get the input and updates from user
//     let res = event_manager.handle_events(&mut pointer, &mut ui_manager);
//     if res == Ok(true) {
//         break 'main;
//     }

//     // Check if user triggered some ui events
//     ui_manager.pointer_collision(&mut pointer, &mut actions, &mut ui_map);

//     // Apply the actions, registered by the user
//     actions.apply(&mut canvas_manager, &pointer);

//     // Update the UI layout if nessesary
//     ui_manager.update(&mut ui_map);

//     // Update canvas, if layout changed, use tool if needed
//     canvas_manager.update(&pointer, &mut ui_map, &mut canvas, &mut texture_manager);

//     // Draw the UI
//     ui_manager.draw_ui(&mut canvas, &ui_map, &texture_manager);

//     canvas.draw_
//     //tell the data, that the frame is over
//     pointer.reset();

//     // buffer draw textures
//     if lazy_buffer.elapsed() >= Duration::from_millis(20) {
//         texture_manager.buffer_draw_texture();
//         lazy_buffer = std::time::Instant::now();
//     }

//     // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 200));
//     // fps.delay();

//     // fps lock
//     frames += 1;
//     let elapsed = last_frame.elapsed();
//     if elapsed < Duration::new(0, 1_000_000_000u32 / 1000) {
//         // NOTE: this does not give exactly 1000 fps, probably bc it sleeps more than 1 ms
//         std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000) - elapsed);
//     }
//     // fps counter
//     last_frame = std::time::Instant::now();
//     if time.elapsed() >= Duration::from_secs(5) {
//         println!("fps: {}", frames / 5);
//         time = std::time::Instant::now();
//         frames = 0;
//     }
// }
//     Ok(())
// }
/*
 I have a drawing app where I want to draw primitives to a texture, and then keep it around as a history, copying it as needed.
The problem is that when the window is resized, all  the textures are blank.
I found [this stackowerflow](https://stackoverflow.com/questions/62600322/sdl-texture-renders-black-after-resize-unless-it-is-redrawn) which explains that target textures are not suitable for storage, especially for Windows + direct3d, which is my case.

So my question is what is the usual way to store texture data for longer.

Here is minimal example:

```
pub fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;
    let video_subsystem = sdl.video()?;

    let mut window = video_subsystem
        .window("Quick Board", 800, 600)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let mut texture = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 400, 300)
        .unwrap();

    canvas.with_texture_canvas(&mut texture, |c| {
        c.set_draw_color(Color::RGB(200, 200, 0));
        c.fill_rect(Rect::new(0, 0, 200, 300));
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
        canvas.copy(
            &texture,
            Rect::new(0, 0, 400, 300),
            Rect::new(100, 100, 400, 300),
        );
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        canvas.window_mut().set_size(1000, 700);
        // screen is completely black after this
    }
    Ok(())
}

```
*/
