use super::{
    canvas_manager::{CanvasData, CanvasManager},
    coords::XY,
    history_step::HistoryStep,
    pointer_state::*,
    texture_manager::*,
};
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::*, video::Window};

use super::tool_trait::ToolTrait;

pub struct Brush {
    brush_size: i16,
    draw_gap: u32,
    last_stroke_at: XY,
}
impl Brush {
    pub fn new() -> Self {
        Self {
            brush_size: 10,
            draw_gap: 0,
            last_stroke_at: XY::new(0, 0),
        }
    }
}
impl ToolTrait for Brush {
    fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        stroke_at: XY,
        pointer: &PointerState,
        canvas: &mut Canvas<Window>,
        t_manager: &mut TextureManager,
    ) {
        if pointer.left == ButtonState::Pressed {
            let step_id = data.history.add_step();
            let step = &mut data.history.steps[step_id];
            let texture_unit = step.get_texture(stroke_at, t_manager);
            let mut texture = t_manager.draw_texture(texture_unit.id);
            canvas.with_texture_canvas(&mut texture, |c| {
                let color = Color::RGB(255, 0, 0);
                c.filled_circle(
                    stroke_at.x as i16,
                    stroke_at.y as i16,
                    self.brush_size,
                    color,
                )
                .unwrap();
            });
        }
        if pointer.left == ButtonState::Held {
            let curr_step = &mut data.history.selected_step_mut();
            let texture_unit = curr_step.get_texture(stroke_at, t_manager);
            let mut texture = t_manager.draw_texture(texture_unit.id);
            canvas.with_texture_canvas(&mut texture, |c| {
                let color = Color::RGB(255, 0, 0);
                c.filled_circle(
                    stroke_at.x as i16,
                    stroke_at.y as i16,
                    self.brush_size,
                    color,
                )
                .unwrap();
            });
        }
    }
}
