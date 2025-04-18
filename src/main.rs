extern crate sdl2;

mod model;
mod relay;
mod view;

use std::time::Duration;

use crate::view::app::App;
use relay::action_pump::ActionPump;
use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};

const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 100;
const COLOR: Color = Color::RGB(200, 200, 200);
const RADIUS: i16 = 3;

pub fn main() -> Result<(), String> {
    // let t_creator: TextureCreator<WindowContext> = app.canvas.texture_creator();

    // TODO: remove
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
        if let Err(e) = res {
            break 'main;
        }

        // Check if user triggered some ui events
        app.ui_manager.pointer_collision(&mut app.states);

        // Apply the actions, registered by the user
        ActionPump::apply(&mut app);

        // Update the UI layout if nessesary
        app.ui_manager.update(&mut app.states);

        // Draw the UI
        app.canvas.set_draw_color(Color::RGB(0, 0, 0));
        app.canvas.clear();
        app.ui_manager.draw_ui(&mut app.canvas);
        app.canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));

        //reset all the states
        app.states.reset();
    }
    Ok(())
}
