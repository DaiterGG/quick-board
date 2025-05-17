#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate indices;
extern crate sdl2;

mod app;
mod debug;

use std::{env, time::*};

use app::{
    action_pump::ActionPump, canvas_manager::CanvasManager, coords::WH, cursor::CanvasCursor,
    event_manager::EventManager, input_state::InputState, predefined::Id, texture_manager::*,
    ui_manager::UIManager, ui_map::UIMap,
};
use sdl2::{image::*, video::*};

pub fn main() -> Result<(), String> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let mut event_manager = EventManager::new(&sdl)?;
    let mut window_size = if let Ok(DisplayMode { w, h, .. }) = video_subsystem.display_mode(0, 0) {
        WH {
            w: (w as f32 * 0.60) as i32,
            h: (h as f32 * 0.60) as i32,
        }
    } else {
        WH { w: 1920, h: 1080 }
    };
    let window = video_subsystem
        .window("Quick Board", window_size.w as u32, window_size.h as u32)
        // .maximized()
        .position_centered()
        .resizable()
        .opengl()
        // .vulkan()
        .build()
        .map_err(|e| e.to_string())?;

    //update window size if it is maximized
    let ws = window.size();
    window_size = WH {
        w: ws.0 as i32,
        h: ws.1 as i32,
    };

    let mut input = InputState::new();
    let mut ui_manager = UIManager::new(window_size);
    let mut actions = ActionPump::new();
    // TODO: move video_subsystem to texture manager
    let mut texture_manager = TextureManager::new(&video_subsystem, window);
    let mut ui_map = UIMap::new();
    let mut canvas_manager =
        CanvasManager::new(&mut texture_manager, &mut ui_map, Id::DrawWindow as i32);

    // let mut fps = FPSManager::new();
    // println!("err {:?}", fps.set_framerate(200));

    let mut time = Instant::now();
    let mut last_frame = Instant::now();
    let mut lazy_buffer = Instant::now();
    let mut frames = 0;
    'main: loop {
        // Get the input and updates from user
        let res = event_manager.handle_events(&mut input, &mut ui_manager, &mut actions);
        if res == Ok(true) {
            break 'main;
        }

        // Check if user triggered some ui events
        ui_manager.pointer_collision(&mut input, &mut actions, &mut ui_map);

        // Apply the actions, registered by the user
        actions.apply(
            &mut canvas_manager,
            &mut ui_manager,
            &input,
            &mut texture_manager,
        );

        // Update the UI layout if nessesary
        ui_manager.update(&mut ui_map);

        // Update canvas, if layout changed, use tool if needed
        canvas_manager.update(&input, &mut ui_map, &mut texture_manager);

        // Draw the UI
        ui_manager.draw_ui(&ui_map, &mut texture_manager);

        //tell the data, that the frame is over
        input.reset();

        // buffer draw textures
        if lazy_buffer.elapsed() >= Duration::from_millis(20) {
            texture_manager.buffer_draw_texture();
            lazy_buffer = Instant::now();
        }

        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 200));
        // fps.delay();

        // fps lock
        frames += 1;
        let elapsed = last_frame.elapsed();
        if elapsed < Duration::new(0, 1_000_000_000u32 / 1000) {
            // NOTE: this does not give exactly 1000 fps, probably bc it sleeps more than 1 ms
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000) - elapsed);
        }
        // fps counter
        last_frame = Instant::now();
        if time.elapsed() >= Duration::from_secs(5) {
            println!("fps: {}", frames / 5);
            time = Instant::now();
            frames = 0;
        }
    }
    Ok(())
}
