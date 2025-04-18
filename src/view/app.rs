use std::time::Duration;

use crate::relay;
use relay::action_pump::ActionPump;

use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    pixels::Color,
    render::{Canvas, CanvasBuilder, TextureCreator},
    video::{DisplayMode, Window, WindowContext},
};

use super::{coords::WH, event_manager::EventManager, states::States, ui_manager::UIManager};

// NOTE:
/// initialized on program start
/// has modules and states
/// modules are called in order in a main loop
/// each module can cange it's data, and any data of another module or state
/// perfect module do not have any data and do not change other module's data
/// at the end of the loop, states are .reset() to change frame dependent data
pub struct App {
    pub states: States,
    pub canvas: Canvas<Window>,
    pub event_manager: EventManager,
    pub ui_manager: UIManager,
    pub sdl: Sdl,
    pub t_creator: TextureCreator<WindowContext>,
    pub video_subsystem: VideoSubsystem,
    pub biggest_possible_resolution: WH,
}
impl App {
    pub fn init() -> Result<App, String> {
        // TODO: read from file
        // let saved_ctx = read_cfg();
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let event_manager = EventManager::new(&sdl)?;
        let mut window_size =
            if let Ok(DisplayMode { w, h, .. }) = video_subsystem.display_mode(0, 0) {
                WH {
                    w: (w as f32 * 0.60) as i32,
                    h: (h as f32 * 0.60) as i32,
                }
            } else {
                WH { w: 1920, h: 1080 }
            };

        let window = video_subsystem
            .window("foo", window_size.w as u32, window_size.h as u32)
            // .maximized()
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        //update window size if it is maximized
        let ws = window.size();
        window_size = WH {
            w: ws.0 as i32,
            h: ws.1 as i32,
        };

        let biggest_possible_resolution = Self::init_biggest_possible_display_res(&video_subsystem);

        let canvas: Canvas<Window> = CanvasBuilder::new(window)
            .build()
            .map_err(|e| e.to_string())?;
        let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let ui_manager = UIManager::new();
        let states = States::new(window_size);

        Ok(App {
            t_creator,
            states,
            biggest_possible_resolution,
            event_manager,
            ui_manager,
            sdl,
            canvas,
            video_subsystem,
        })
    }
    pub fn get_biggest_possible_resolution(&self) -> WH {
        self.biggest_possible_resolution
    }
    pub fn update_biggest_possible_display_res(&mut self) {
        self.biggest_possible_resolution =
            Self::init_biggest_possible_display_res(&self.video_subsystem);
    }
    fn init_biggest_possible_display_res(video_subsystem: &VideoSubsystem) -> WH {
        let mut max_wh = WH { w: 1000, h: 500 };
        let count = video_subsystem.num_video_displays().unwrap_or(1);
        for i in 0..count {
            // TODO: find what mode_index is
            if let Ok(res) = video_subsystem.display_mode(i, 0) {
                if res.w > max_wh.w {
                    max_wh.w = res.w;
                }
                if res.h > max_wh.h {
                    max_wh.h = res.h;
                }
            }
        }
        max_wh
    }
    pub fn sdl(&self) -> &Sdl {
        &self.sdl
    }
    pub fn video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }
}

/// for testing
impl Default for App {
    fn default() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let canvas = CanvasBuilder::new(video_subsystem.window("foo", 1920, 1080).build().unwrap())
            .build()
            .unwrap();
        Self {
            states: States::new(WH { w: 1920, h: 1080 }),
            ui_manager: UIManager::new(),
            t_creator: canvas.texture_creator(),
            event_manager: EventManager::new(&sdl).unwrap(),
            sdl,
            video_subsystem,
            canvas,
            biggest_possible_resolution: WH { w: 1920, h: 1080 },
        }
    }
}
