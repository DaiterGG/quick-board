use std::cmp::*;

use crate::*;

use super::{canvas_manager::*, coords::*, pointer_state::*, texture_manager::*};
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::*, video::Window};

pub struct Brush {
    brush_diameter: i32,
    draw_gap: i32,
    last_stroke_at: XY,
    color: Color,
    brush_texture_id: Option<usize>,
}
impl Brush {
    pub fn new(t_manager: &mut TextureManager) -> Self {
        let brush_diameter = 300;
        let size = WH::new(brush_diameter, brush_diameter);
        let brush_texture_id = Some(t_manager.init_open_texture(Some(size)));
        Self {
            brush_diameter: 300,
            draw_gap: 1,
            last_stroke_at: XY::new(0, 0),
            color: Color::RGBA(200, 200, 200, 255),
            brush_texture_id,
        }
    }
    pub fn update_texture(&mut self, t_manager: &mut TextureManager) {
        if self.brush_texture_id.is_none() {
            let size = WH::new(self.brush_diameter, self.brush_diameter);
            self.brush_texture_id = Some(t_manager.init_open_texture(Some(size)));
        }
    }
    pub fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        pointer: &PointerState,
        stroke_at: XY,
        canvas: &mut Canvas<Window>,
        t_manager: &mut TextureManager,
    ) {
        if pointer.left == ButtonState::Idle
            || pointer.interacting_with != Some(data.targeted_ui_element)
        {
            return;
        }
        if pointer.left == ButtonState::Released {
            data.history.finish_step(t_manager, canvas);
            return;
        }

        let radius = (self.brush_diameter / 2) as i32;

        if pointer.left == ButtonState::Pressed {
            data.history.add_step();
            let step = &mut data.history.selected_step_mut();
            self.last_stroke_at = stroke_at;
            let bound = stroke_at.expand(radius, radius);
            let texture_unit_vec = step.get_textures(bound, t_manager);
            for tex in 0..texture_unit_vec.len() {
                let origin = texture_unit_vec[tex].origin;
                let texture = t_manager.draw_texture(texture_unit_vec[tex].id);
                self.draw_to_texture(stroke_at.substract(origin), canvas, texture);
            }
            return;
        }
        let step = &mut data.history.selected_step_mut();

        let traveled = stroke_at.distance(self.last_stroke_at);

        let gap_f = self.draw_gap as f32;
        let circles_count_f = traveled / gap_f;
        let circles_count = circles_count_f.floor() as i32;
        if circles_count == 0 {
            return;
        }
        let gap_x = (stroke_at.x - self.last_stroke_at.x) as f32 / circles_count_f;
        let gap_y = (stroke_at.y - self.last_stroke_at.y) as f32 / circles_count_f;

        let mut circles = Vec::new();
        let mut new_last_stroke = self.last_stroke_at;
        for i in 0..circles_count {
            circles.push(XY::new(
                self.last_stroke_at.x + (gap_x * (i + 1) as f32) as i32,
                self.last_stroke_at.y + (gap_y * (i + 1) as f32) as i32,
            ));
            let first = circles[0];
            let last = circles[circles.len() - 1];
            if (first.x - last.x).abs() > DRAW_TEX_SIZE_I32 * 2
                || (first.y - last.y).abs() > DRAW_TEX_SIZE_I32 * 2
                || i == circles_count - 1
            {
                let stroke_box = first.to_bound(last).expand(radius, radius);
                if !data.transform.to_bound().is_overlaping(stroke_box) {
                    new_last_stroke = stroke_at;
                    break;
                }

                let texture_unit_vec = step.get_textures(stroke_box, t_manager);

                for i in 0..circles.len() {
                    let circle_at = circles[i];
                    for tex in 0..texture_unit_vec.len() {
                        let origin = texture_unit_vec[tex].origin;
                        let texture = t_manager.draw_texture(texture_unit_vec[tex].id);
                        self.draw_to_texture(circle_at.substract(origin), canvas, texture);
                    }
                }
                new_last_stroke = last;
                circles.clear();
            }
        }
        self.last_stroke_at = new_last_stroke;
    }
    fn draw_to_texture(&mut self, pos: XY, canvas: &mut Canvas<Window>, texture: &mut Texture) {
        let _ = canvas.with_texture_canvas(texture, |c| {
            if self.brush_diameter == 1 {
                let _ = c.pixel(pos.x as i16, pos.y as i16, self.color);
                return;
            }
            if self.brush_diameter > 1000 {
                let _ = c.filled_pie(
                    pos.x as i16,
                    pos.y as i16,
                    (self.brush_diameter / 2) as i16,
                    -1,
                    359,
                    self.color,
                );
                return;
            }
            let _ = c.filled_circle(
                pos.x as i16,
                pos.y as i16,
                (self.brush_diameter / 2) as i16,
                self.color,
            );
        });
    }
    pub fn change_brush_size(&mut self, up: bool) {
        let by = if up { 1.1 } else { 0.9 };
        self.brush_diameter = (self.brush_diameter as f32 * by) as i32;
    }
}
