use super::coords::WH;

pub struct UIState {
    pub requires_update: bool,
    pub window_size: WH,
    pub custom_ui_scale: Option<f32>,
}

impl UIState {
    pub fn new(window_size: WH) -> Self {
        Self {
            window_size,
            requires_update: true,
            custom_ui_scale: None,
        }
    }
    pub fn get_current_ui_scale(&self) -> f32 {
        match self.custom_ui_scale {
            Some(x) => x,
            None => self.window_size.h as f32 / 1080f32,
        }
    }
    pub fn reset(&mut self) {}
}
