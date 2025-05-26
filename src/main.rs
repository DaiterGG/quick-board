// #![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate indices;
extern crate sdl2;

mod app;
mod debug;

use std::{env, path::Path, thread::sleep, time::*};

use app::{
    action_pump::*, canvas_manager::CanvasManager, coords::WH, cursor::CursorManager,
    event_manager::EventManager, input_state::InputState, predefined::Id, texture_manager::*,
    ui_manager::UIManager, ui_map::UIMap,
};
use sdl2::{
    VideoSubsystem,
    image::*,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    video::*,
};

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
        CanvasManager::new(&mut texture_manager, &mut ui_map, Id::DrawWindow as i32);
    let cursor_manager = CursorManager::new(
        canvas_manager.data.screen_zoom,
        canvas_manager.tools.get_size(canvas_manager.current_tool),
        bpr,
    );
    let mut input = InputState::new(cursor_manager);

    // let mut fps = FPSManager::new();
    // println!("err {:?}", fps.set_framerate(200));

    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // let mut font = ttf_context.load_font("../resources/fonts/inter.ttf", 128)?;
    // font.set_style(sdl2::ttf::FontStyle::BOLD);
    // let surface = font
    //     .render("Hello Rust!")
    //     .blended(Color::RGBA(255, 0, 0, 255))
    //     .map_err(|e| e.to_string())?;
    // let texture = texture_manager
    //     .t_creator
    //     .create_texture_from_surface(&surface)
    //     .map_err(|e| e.to_string())?;
    // texture_manager.canvas.copy(&texture, None, None)?;
    // texture_manager.canvas.present();
    // sleep(Duration::from_secs(1));

    let mut time = Instant::now();
    let mut last_frame = Instant::now();
    let mut lazy_buffer = Instant::now();
    let mut frames = 0;
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
            &mut input,
            &mut texture_manager,
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
            // println!("fps: {}", frames / 5);
            time = Instant::now();
            frames = 0;
        }
    }
    Ok(())
}
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
