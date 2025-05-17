use super::{canvas_manager::CanvasData, coords::*, input_state::InputState};

pub struct Move {
    last_strocke_at: Option<XY>,
}
impl Move {
    pub fn new() -> Self {
        Self {
            last_strocke_at: None,
        }
    }
    pub fn process_stroke(&mut self, data: &mut CanvasData, input: &InputState) {
        if input.interacting_with == Some(data.targeted_ui_element) {
            if let Some(last) = self.last_strocke_at {
                data.screen_pos.x += input.pos.x - last.x;
                data.screen_pos.y += input.pos.y - last.y;
            }
            self.last_strocke_at = Some(input.pos);
        } else {
            self.last_strocke_at = None;
        }
        // println!("{:?}", self.last_strocke_at);
        if input.scroll_y == 0 {
            return;
        }

        let direction = if input.scroll_y > 0 { 1.0 } else { -1.0 };
        let zoom_by = data.screen_zoom * 0.1 * direction;
        let before_x = (input.pos.x - data.screen_pos.x) as f32 / data.screen_zoom;
        let before_y = (input.pos.y - data.screen_pos.y) as f32 / data.screen_zoom;
        data.screen_zoom += zoom_by;
        let dif_x = before_x - (input.pos.x - data.screen_pos.x) as f32 / data.screen_zoom;
        let dif_y = before_y - (input.pos.y - data.screen_pos.y) as f32 / data.screen_zoom;
        let ui_dif = XY {
            x: (dif_x * data.screen_zoom) as i32,
            y: (dif_y * data.screen_zoom) as i32,
        };
        data.screen_pos = data.screen_pos.substract(ui_dif);
        data.update_cursor = true;
    }
}
