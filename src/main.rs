extern crate sdl2;

mod app;

use std::{env, time::Duration};

use app::{
    action_pump::ActionPump, canvas_manager::CanvasManager, coords::WH,
    event_manager::EventManager, pointer_state::PointerState, predefined::Id, texture_manager::*,
    ui_manager::UIManager, ui_map::UIMap,
};
use sdl2::{render::*, video::*};

pub fn main() -> Result<(), String> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
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

    let mut canvas: Canvas<Window> = CanvasBuilder::new(window)
        // .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    // WindowState::new(sdl, video_subsystem, canvas, t_creator),

    let mut pointer = PointerState::new();
    let mut ui_manager = UIManager::new(window_size);
    let mut actions = ActionPump::new();
    // TODO: move video_subsystem to texture manager
    let mut texture_manager = TextureManager::new(t_creator, &video_subsystem);
    let mut ui_map = UIMap::new();
    let mut canvas_manager =
        CanvasManager::new(&mut texture_manager, &mut ui_map, Id::DrawWindow as usize);

    // canvas.with_texture_canvas(
    //     &mut texture_manager
    //         .predefined_mut(TextureId::DrawCanvas)
    //         .texture,
    //     |c| {
    //         c.set_draw_color(Color::RGB(255, 255, 255));
    //         c.clear();
    //     },
    // );
    // let mut fps = FPSManager::new();
    // println!("err {:?}", fps.set_framerate(200));
    // println!("fps: {:?}", fps.get_framerate());
    let mut time = std::time::Instant::now();
    let mut last_frame = std::time::Instant::now();
    let mut frames = 0;
    'main: loop {
        // Get the input and updates from user
        let res = event_manager.handle_events(&mut pointer, &mut ui_manager);
        if res == Ok(true) {
            break 'main;
        }

        // Check if user triggered some ui events
        ui_manager.pointer_collision(&mut pointer, &mut actions, &mut ui_map);

        // Apply the actions, registered by the user
        actions.apply(&mut canvas_manager, &pointer);

        // Update the UI layout if nessesary
        ui_manager.update(&mut ui_map);

        // Update canvas, if layout changed, use tool if needed
        canvas_manager.update(&pointer, &mut ui_map, &mut canvas, &mut texture_manager);

        // Draw the UI
        ui_manager.draw_ui(&mut canvas, &ui_map, &texture_manager);

        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 200));

        //tell the data, that the frame is over
        pointer.reset();

        // fps.delay();
        frames += 1;
        let elapsed = last_frame.elapsed();
        if elapsed < Duration::new(0, 1_000_000_000u32 / 1000) {
            // NOTE: this does not give exactly 1000 fps, probably bc it sleeps more than 1 ms
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000) - elapsed);
        }
        // println!("{:?}", elapsed);
        last_frame = std::time::Instant::now();
        if time.elapsed() >= Duration::from_secs(5) {
            println!("fps: {}", frames / 5);
            time = std::time::Instant::now();
            frames = 0;
        }
    }
    Ok(())
}
