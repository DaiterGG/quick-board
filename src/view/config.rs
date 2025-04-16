use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    render::{Canvas, CanvasBuilder},
    video::{DisplayMode, Window},
};

use super::{
    coords::{WH, XY},
    event_manager::EventManager,
    style_map::StyleMap,
};

/// sigleton object
/// initialized on program start
/// fetches cfg from file
/// provides current ui configuration
/// and api to change it
/// handle writing to file
pub struct Config {
    custom_ui_scale: Option<i32>,
    pub canvas: Canvas<Window>,
    event_manager: EventManager,
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    biggest_possible_resolution: WH,
    styles: StyleMap,
    window_size: WH,
}
impl Config {
    pub fn init() -> Result<Config, String> {
        // TODO: read from file
        // let saved_ctx = read_cfg();
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let event_manager = EventManager::new(&sdl)?;
        let mut window_size =
            if let Ok(DisplayMode { w, h, .. }) = video_subsystem.display_mode(0, 0) {
                WH {
                    w: (w as f32 * 0.85) as i32,
                    h: (h as f32 * 0.90) as i32,
                }
            } else {
                WH { w: 1920, h: 1080 }
            };

        let window = video_subsystem
            .window("foo", window_size.w as u32, window_size.h as u32)
            .maximized()
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

        let canvas: Canvas<Window> = CanvasBuilder::new(window)
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Config {
            event_manager,
            custom_ui_scale: None,
            biggest_possible_resolution: Self::init_biggest_possible_display_res(&video_subsystem),
            window_size,
            styles: StyleMap::new_first(),
            sdl,
            canvas,
            video_subsystem,
        })
    }
    pub fn get_current_ui_scale(&self) -> i32 {
        match self.custom_ui_scale {
            Some(x) => x,
            None => self.window_size().h / 1080,
        }
    }
    pub fn get_biggest_possible_resolution(&self) -> WH {
        self.biggest_possible_resolution
    }
    // pub fn get_current_window_size(&self) -> XY {
    //     self.window_size
    // }
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
    pub fn styles(&self) -> &StyleMap {
        &self.styles
    }
    pub fn sdl(&self) -> &Sdl {
        &self.sdl
    }
    pub fn video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }
    pub fn window_size(&self) -> WH {
        self.window_size
    }
    pub fn resize_window(&mut self, ws: WH) {
        // TODO: resize
        self.window_size = ws;
    }
}

/// for testing
impl Default for Config {
    fn default() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        Self {
            event_manager: EventManager::new(&sdl).unwrap(),
            custom_ui_scale: None,
            sdl,
            canvas: CanvasBuilder::new(video_subsystem.window("foo", 1920, 1080).build().unwrap())
                .build()
                .unwrap(),
            video_subsystem,
            window_size: WH {
                w: (1920.0 * 0.8) as i32,
                h: 1000,
            },
            biggest_possible_resolution: WH { w: 1920, h: 1080 },
            styles: StyleMap::default(),
        }
    }
}
