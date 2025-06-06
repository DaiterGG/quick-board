// to disable (console + app) mode on release builds for windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate indices;
extern crate sdl2;
mod app;
mod debug;

use std::{env, time::*};

use app::{
    action_pump::*, canvas_manager::CanvasManager, coords::WH, cursor::CursorManager,
    event_manager::EventManager, input_state::InputState, predefined::Id, texture_manager::*,
    ui_manager::UIManager, ui_map::UIMap,
};
use sdl2::{VideoSubsystem, image::*, render::BlendMode, video::*};

pub fn main() -> Result<(), String> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let mut event_manager = EventManager::new(&sdl)?;
    let window_size = if let Ok(DisplayMode { w, h, .. }) = video_subsystem.display_mode(0, 0) {
        WH {
            w: (w as f32 * 0.60) as i32,
            h: (h as f32 * 0.60) as i32,
        }
    } else {
        WH { w: 1920, h: 1080 }
    };

    let bpr = init_biggest_possible_display_res(&video_subsystem);

    let window = video_subsystem
        .window("Quick Board", window_size.w as u32, window_size.h as u32)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    ActionPump::init();

    let mut texture_manager = TextureManager::new(window, bpr);
    let mut ui_manager = UIManager::new(texture_manager.canvas.window().size().1);
    let mut ui_map = UIMap::new();
    let mut canvas_manager =
        CanvasManager::new(&mut texture_manager, &mut ui_map, Id::DrawWindow.into());
    let cursor_manager = CursorManager::new(
        canvas_manager.data.screen_zoom,
        canvas_manager.tools.get_size(canvas_manager.current_tool),
        bpr,
    );
    let mut input = InputState::new(cursor_manager);

    let mut time = Instant::now();
    let mut last_frame = Instant::now();
    let mut lazy_buffer = Instant::now();
    let mut frames_fps = 0;
    let mut frames_counter = 0;
    //seconds between frames
    let mut delta = 0.001;
    'main: loop {
        // Get the input and updates from user
        let res =
            event_manager.handle_events(&mut input, &mut ui_manager, &mut texture_manager, &sdl);
        if res == Ok(true) {
            break 'main;
        }

        // Check if user triggered some ui events
        ui_manager.pointer_collision(&mut input, &mut ui_map);

        // Apply the actions, registered by the user
        ActionPump::apply(
            &mut canvas_manager,
            &mut ui_manager,
            &mut ui_map,
            &mut input,
            &mut texture_manager,
            delta,
            frames_counter,
        );

        // Update the UI layout if nessesary
        ui_manager.update(&mut ui_map, &mut texture_manager);

        // Update canvas, if layout changed, use tool if needed
        canvas_manager.update(&mut input, &mut ui_map, &mut texture_manager);

        // Draw the UI
        ui_manager.draw_ui(&ui_map, &mut texture_manager);

        // buffer draw textures
        if lazy_buffer.elapsed() >= Duration::from_millis(20) {
            texture_manager.buffer_draw_texture();
            lazy_buffer = Instant::now();
        }

        //tell the data, that the frame is over
        input.reset();

        // sdl.mouse()
        //     .warp_mouse_in_window(&texture_manager.canvas.window(), 50, 50);

        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 200));
        // fps.delay();

        frames_fps += 1;
        frames_counter += 1;
        // fps lock
        let elapsed = last_frame.elapsed();
        if elapsed < Duration::new(0, 1_000_000_000u32 / 1000) {
            // NOTE: this does not give exactly 1000 fps, bc sleep is not precise
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000) - elapsed);
        }
        delta = last_frame.elapsed().as_secs_f32();
        if delta == 0.0 {
            delta = 0.001;
        }
        // fps counter
        last_frame = Instant::now();
        if time.elapsed() >= Duration::from_secs(5) {
            println!("fps: {}", frames_fps / 5);
            time = Instant::now();
            frames_fps = 0;
        }
    }
    Ok(())
}
// ui buffer textures has to be some size that will be sufficient for biggest display user has
// NOTE: to prevent changing landscape and portrait modes, it is always a big square
// FIXME: if possibel track such changes in event manager, and update every buffer
fn init_biggest_possible_display_res(video_subsystem: &VideoSubsystem) -> i32 {
    let mut max_wh = -1;
    let count = video_subsystem.num_video_displays().unwrap_or(1);
    for i in 0..count {
        for mode in 0..video_subsystem.num_display_modes(i).unwrap_or(1) {
            if let Ok(res) = video_subsystem.display_mode(i, mode) {
                if res.w > max_wh {
                    max_wh = res.w;
                }
                if res.h > max_wh {
                    max_wh = res.h;
                }
            }
        }
    }
    if max_wh == -1 {
        max_wh = 1920;
    }
    max_wh
}
