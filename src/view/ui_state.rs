use super::coords::WH;

pub struct UIState {
    pub window_size: WH,
    pub custom_ui_scale: Option<f32>,
    requires_update: bool,
    was_updated_last_frame: bool,
}

impl UIState {
    pub fn new(window_size: WH) -> Self {
        Self {
            window_size,
            requires_update: true,
            was_updated_last_frame: false,
            custom_ui_scale: None,
        }
    }
    pub fn get_current_ui_scale(&self) -> f32 {
        match self.custom_ui_scale {
            Some(x) => x,
            None => self.window_size.h as f32 / 1080f32,
        }
    }
    pub fn requires_update(&self) -> bool {
        self.requires_update
    }
    pub fn now_is_require_update(&mut self) {
        self.requires_update = true;
    }
    pub fn was_updated_last_frame(&self) -> bool {
        self.was_updated_last_frame
    }
    pub fn reset(&mut self) {
        if self.requires_update {
            self.was_updated_last_frame = true;
            self.requires_update = false;
        } else {
            self.was_updated_last_frame = false;
        }
    }
}
