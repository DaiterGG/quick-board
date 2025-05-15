use std::{cmp::*, thread::sleep};

use crate::*;

use super::{
    canvas_manager::*, coords::*, pointer_state::*, texture_data::TextureData, texture_manager::*,
};
use indices::indices;
use sdl2::{gfx::primitives::DrawRenderer, pixels::*, rect::Rect, render::*, video::Window};

const BUFFER_TEX_SIZE: i32 = 5000;
const BUFFER_TEX_SIZE_I16: i16 = BUFFER_TEX_SIZE as i16;
pub struct Brush {
    brush_diameter: i32,
    draw_gap: i32,
    last_stroke_at: XY,
    color: Color,
    mask_id: usize,
    // the primary reason why the is a buffer is to buffer the "scale down" operation
    // of potentially big mask
    buffer_id: usize,
}
impl Brush {
    pub fn new(t_manager: &mut TextureManager) -> Self {
        let diameter = 1;
        // let size = WH::new(brush_diameter, brush_diameter);
        let size = WH::new(BUFFER_TEX_SIZE, BUFFER_TEX_SIZE);

        let mask_id =
            t_manager.init_open_texture(TextureData::new(&t_manager.t_creator, size, None));
        t_manager.open_textures[mask_id]
            .texture
            .set_blend_mode(BlendMode::Blend);

        let buffer_id =
            t_manager.init_open_texture(TextureData::new(&t_manager.t_creator, size, None));
        t_manager.open_textures[buffer_id]
            .texture
            .set_blend_mode(BlendMode::Blend);

        let mut s = Self {
            buffer_id,
            brush_diameter: diameter,
            draw_gap: 1,
            last_stroke_at: XY::new(0, 0),
            color: Color::RGBA(255, 0, 0, 255),
            mask_id,
        };
        s.generate_circle_mask(t_manager);
        // t_manager.open_textures[s.buffer_id].src =
        //     Some(XYWH::new(0, 0, s.brush_diameter, s.brush_diameter));
        s.update_buffer(t_manager);
        s
    }
    fn generate_circle_mask(&mut self, t_manager: &mut TextureManager) {
        let _ = t_manager.canvas.with_texture_canvas(
            &mut t_manager.open_textures[self.mask_id].texture,
            |c| {
                //create a fake original data
                c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                c.clear();
                c.filled_pie(
                    BUFFER_TEX_SIZE_I16 / 2,
                    BUFFER_TEX_SIZE_I16 / 2,
                    BUFFER_TEX_SIZE_I16 / 2,
                    -1,
                    359,
                    Color::RGBA(255, 255, 255, 255),
                );
            },
        );
    }

    fn update_buffer(&mut self, t_manager: &mut TextureManager) {
        let src = Rect::new(0, 0, self.brush_diameter as u32, self.brush_diameter as u32);

        let (buf_t, mask_t) = indices!(&mut t_manager.open_textures, self.buffer_id, self.mask_id);

        t_manager
            .canvas
            .with_texture_canvas(&mut buf_t.texture, |c| {
                c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                c.clear();
                c.copy(&mask_t.texture, None, src);
            });
    }

    // no texture recreation for now
    pub fn resize(&mut self, t_manager: &mut TextureManager, new_size: i32) {
        // self.brush_diameter = new_size;
        // let size = WH::new(self.brush_diameter, self.brush_diameter);
        // t_manager.destroy_open_texture(self.buffer_id);
        // self.buffer_id =
        //     t_manager.init_open_texture(TextureData::new(&t_manager.t_creator, size, None));
        self.brush_diameter = new_size;
        // t_manager.open_textures[self.buffer_id].src =
        //     Some(XYWH::new(0, 0, self.brush_diameter, self.brush_diameter));
        self.update_buffer(t_manager);
    }
    pub fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        pointer: &PointerState,
        stroke_at: XY,
        t_manager: &mut TextureManager,
    ) {
        if pointer.left == ButtonState::Idle
            || pointer.interacting_with != Some(data.targeted_ui_element)
        {
            return;
        }
        if pointer.left == ButtonState::Released {
            data.history.finish_step(t_manager);
            return;
        }

        let radius = self.brush_diameter / 2;

        if pointer.left == ButtonState::Pressed {
            data.history.add_step();
            let step = &mut data.history.selected_step_mut();
            self.last_stroke_at = stroke_at;
            let bound = stroke_at.expand(radius, radius);
            let texture_unit_vec = step.get_textures(bound, data.transform, t_manager);
            let buffer = &mut t_manager.open_textures[self.buffer_id];
            let buffer_at = stroke_at.substract_one(radius);
            for tex in 0..texture_unit_vec.len() {
                let origin = texture_unit_vec[tex].origin;
                let texture = &mut t_manager.draw_textures[texture_unit_vec[tex].id];
                self.draw_to_texture(
                    buffer_at.substract(origin),
                    &mut t_manager.canvas,
                    texture,
                    buffer,
                );
            }
            return;
        }
        let step = &mut data.history.selected_step_mut();

        let traveled = stroke_at.distance(self.last_stroke_at);

        let gap_f = self.draw_gap as f32;
        let strokes_count_f = traveled / gap_f;
        let strokes_count = strokes_count_f.floor() as i32;
        if strokes_count == 0 {
            return;
        }
        let gap_x = (stroke_at.x - self.last_stroke_at.x) as f32 / strokes_count_f;
        let gap_y = (stroke_at.y - self.last_stroke_at.y) as f32 / strokes_count_f;

        let mut strokes = Vec::new();
        let mut new_last_stroke = self.last_stroke_at;
        for i in 0..strokes_count {
            strokes.push(XY::new(
                self.last_stroke_at.x + (gap_x * (i + 1) as f32) as i32,
                self.last_stroke_at.y + (gap_y * (i + 1) as f32) as i32,
            ));
            let first = strokes[0];
            let last = strokes[strokes.len() - 1];
            if (first.x - last.x).abs() > DRAW_TEX_SIZE_I32 * 2
                || (first.y - last.y).abs() > DRAW_TEX_SIZE_I32 * 2
                || i == strokes_count - 1
            {
                let stroke_box = first.to_bound(last).expand(radius, radius);
                if !data.transform.to_bound().is_overlaping(stroke_box) {
                    new_last_stroke = stroke_at;
                    break;
                }

                // getting textures is about 50 times faster than drawing on them
                let texture_unit_vec = step.get_textures(stroke_box, data.transform, t_manager);

                let buffer = &mut t_manager.open_textures[self.buffer_id];

                for i in 0..strokes.len() {
                    let buffer_at = strokes[i].substract_one(radius);

                    for tex in 0..texture_unit_vec.len() {
                        let origin = texture_unit_vec[tex].origin;
                        let texture = &mut t_manager.draw_textures[texture_unit_vec[tex].id];
                        let to_unit_coord = buffer_at.substract(origin);
                        self.draw_to_texture(to_unit_coord, &mut t_manager.canvas, texture, buffer);
                    }
                }

                new_last_stroke = last;
                strokes.clear();
            }
        }
        // if time.elapsed().as_nanos() > 10 {}
        self.last_stroke_at = new_last_stroke;
    }
    /// * `buffer_at`: distance from unit origin to buffer onigin
    /// * (buffer pos in unit coordinate space)
    fn draw_to_texture(
        &mut self,
        buffer_at: XY,
        canvas: &mut Canvas<Window>,
        texture: &mut Texture,
        buffer: &mut TextureData,
    ) {
        buffer
            .texture
            .set_color_mod(self.color.r, self.color.g, self.color.b);
        let time = std::time::Instant::now();

        let dst = buffer_at
            .to_tr_one(self.brush_diameter)
            .get_overlap(WH::new_one(DRAW_TEX_SIZE_I32));
        if dst.w == 0 || dst.h == 0 {
            return;
        }

        let src = XYWH::new(-min(buffer_at.x, 0), -min(buffer_at.y, 0), dst.w, dst.h);
        // let time = std::time::Instant::now();

        //if draw_size is 256 - 12,800 us, overlap w 220 h 206 45320
        let _ = canvas.with_texture_canvas(texture, |c| {
            let _ = c.copy(&buffer.texture, Some(src.to_rect()), Some(dst.to_rect()));
        });
        // print!("{}, ", time.elapsed().as_nanos());
        // println!("overlap {} {} {}", dst.w, dst.h, dst.w * dst.h);
    }
    pub fn change_brush_size(&mut self, up: bool, t_manager: &mut TextureManager) {
        let new_size = if up {
            (self.brush_diameter as f32 * 1.1).ceil() as i32
        } else {
            (self.brush_diameter as f32 * 0.9).floor() as i32
        };
        self.resize(t_manager, new_size);
    }
}
