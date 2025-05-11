use std::cmp::*;

use super::{
    canvas_manager::CanvasData, coords::*, history_step::HistoryStep, pointer_state::PointerState,
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
                data.screen_pos.x += pointer.pos.x - last.x;
                data.screen_pos.y += pointer.pos.y - last.y;
            }
            self.last_strocke_at = Some(pointer.pos);
        } else {
            self.last_strocke_at = None;
        }
        // println!("{:?}", self.last_strocke_at);
        if pointer.scroll_y == 0 {
            return;
        }

        let zoom_by = if pointer.scroll_y > 0 {
            data.screen_zoom * 0.1
        } else {
            data.screen_zoom * -0.1
        };
        let before = pointer
            .pos
            .transform_from(data.screen_zoom, data.screen_pos);
        data.screen_zoom += zoom_by;
        let after = pointer
            .pos
            .transform_from(data.screen_zoom, data.screen_pos);
        let dif = before.substract(after);
        let ui_dif = XY {
            x: (dif.x as f32 * data.screen_zoom) as i32,
            y: (dif.y as f32 * data.screen_zoom) as i32,
        };

        data.screen_pos = data.screen_pos.substract(ui_dif);
    }
}
