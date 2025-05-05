use super::{
    canvas_manager::*, coords::*, history_step::HistoryStep, pointer_state::*, texture_manager::*,
    tool_trait::ToolData,
};
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::*, video::Window};

use super::tool_trait::ToolTrait;

pub struct Brush {
    brush_size: i32,
    draw_gap: u32,
    last_stroke_at: XY,
    color: Color,
}
impl Brush {
    pub fn new() -> Self {
        Self {
            brush_size: 10,
            draw_gap: 0,
            last_stroke_at: XY::new(0, 0),
            color: Color::RGB(222, 222, 222),
        }
    }
}
impl ToolTrait for Brush {
    fn process_stroke(&mut self, data: ToolData) {
        if data.pointer.left == ButtonState::Idle {
            return;
        }
        let stroke_at = data
            .pointer
            .pos
            .transform_from(data.c_data.screen_zoom, data.c_data.screen_pos);

        let step = if pointer.left == ButtonState::Pressed {
            let step_id = data.history.add_step();
            &mut data.history.steps[step_id]
        } else {
            &mut data.history.selected_step_mut()
        };
        let stroke_box = XYWH {
            x: stroke_at.x - self.brush_size as i32,
            y: stroke_at.y - self.brush_size as i32,
            w: self.brush_size * 2,
            h: self.brush_size * 2,
        };
        let texture_unit_vec = step.get_textures(stroke_box, t_manager);
        for tex in texture_unit_vec {
            let texture = t_manager.draw_texture(tex.id);
            self.draw_to_texture(stroke_at.substract(tex.origin), canvas, texture);
        }
    }
}
impl Brush {
    fn draw_to_texture(&mut self, pos: XY, canvas: &mut Canvas<Window>, texture: &mut Texture) {
        let _ = canvas.with_texture_canvas(texture, |c| {
            let _ = c.filled_circle(
                pos.x as i16,
                pos.y as i16,
                self.brush_size as i16,
                self.color,
            );
        });
    }
}
