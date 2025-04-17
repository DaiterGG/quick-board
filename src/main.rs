extern crate sdl2;

mod model;
mod relay;
mod view;

use std::time::Duration;

use crate::view::app::App;
use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};

const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 100;
const COLOR: Color = Color::RGB(200, 200, 200);
const RADIUS: i16 = 3;

// TODO:'s
// event file
// event call ui_manager.pointer_moved() recursevly
// event call ctx.update_window_size()
//

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

    let canvas = &mut app.canvas;

    'main: loop {
        // Get the input and updates from user
        let res = app.event_manager.handle_events(&mut app.states);
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.contains("USER_QUIT") {
                    break 'main;
                } else {
                    Err(e)
                }
            }
        }

        // Update the UI layout if nessesary
        app.ui_manager.update(&mut app.states);

        // Apply the actions, registered by the user
        app.action.apply(app.states.action.actions);

        // Draw the UI
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        app.ui_manager.draw_ui(canvas);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
        //reset all the states
    }

    Ok(())
}
