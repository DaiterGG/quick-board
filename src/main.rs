extern crate sdl2;

mod model;
mod relay;
mod view;

use std::time::Duration;

use crate::view::app::App;
use relay::action_pump::ActionPump;
use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};

pub fn main() -> Result<(), String> {
    // TODO: remove
    // let t_creator: TextureCreator<WindowContext> = app.canvas.texture_creator();
    // let mut draw_texture = t_creator
    //     .create_texture_target(
    //         t_creator.default_pixel_format(),
    //         SCREEN_WIDTH * 2,
    //         SCREEN_HEIGHT * 2,
    //     )
    //     .unwrap();
    // draw_texture.set_blend_mode(BlendMode::Blend);

    // let res = ctx.main_loop();

    let mut app = App::init()?;

    'main: loop {
        // Get the input and updates from user
        let res = app.event_manager.handle_events(&mut app.states);
        if let Err(_e) = res {
            break 'main;
        }

        // Check if user triggered some ui events
        app.ui_manager.pointer_collision(&mut app.states);

        // Apply the actions, registered by the user
        ActionPump::apply(&mut app);

        // Update the UI layout if nessesary
        app.ui_manager.update(&mut app.states);

        // Draw the UI
        app.ui_manager.draw_ui(&mut app.canvas);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));

        //tell the data, that the frame is over
        app.states.reset();
    }
    Ok(())
}
