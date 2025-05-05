use std::cmp::*;

use super::{
    canvas_manager::CanvasData, coords::XY, history_step::HistoryStep, pointer_state::PointerState,
    predefined::Id,
};

pub struct Move {
    last_strocke_at: Option<XY>,
}
impl Move {
    pub fn new() -> Self {
        Self {
            last_strocke_at: None,
        }
    }
    pub fn process_stroke(&mut self, data: &mut CanvasData, pointer: &PointerState) {
        if pointer.interacting_with == Some(data.targeted_ui_element) {
            if let Some(last) = self.last_strocke_at {
                // println!("{:?}", pointer.pos.x - last.x);
                data.screen_pos.x += pointer.pos.x - last.x;
                data.screen_pos.y += pointer.pos.y - last.y;
            }
            self.last_strocke_at = Some(pointer.pos);
        } else {
            self.last_strocke_at = None;
        }
        // println!("{:?}", self.last_strocke_at);
        if pointer.scroll_y > 0 {
            data.screen_zoom *= 1.1;
        } else if pointer.scroll_y < 0 {
            data.screen_zoom *= 0.9;
        }
    }
}
