use std::{cmp::*, thread::sleep};

use crate::*;

use super::{
    canvas_manager::*, coords::*, input_state::*, texture_data::TextureData, texture_manager::*,
};
use indices::indices;
use sdl2::{gfx::primitives::DrawRenderer, pixels::*, rect::Rect, render::*, video::Window};

const BUFFER_TEX_SIZE: i32 = 5000;
const BUFFER_TEX_SIZE_I16: i16 = BUFFER_TEX_SIZE as i16;
pub struct Brush {
    // private set
    // pub get sub_brush_diameter as i32
    pub sub_brush_diameter: f32,
    draw_gap_percent: i32,
    last_stroke_at: XY,
    color: Color,
    mask_id: usize,
    // the primary reason why the is a buffer is to buffer the "scale down" operation
    // of potentially big mask
    buffer_id: usize,
}
impl Brush {
    pub fn new(t_manager: &mut TextureManager) -> Self {
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
            sub_brush_diameter: 3.0,
            draw_gap_percent: 10,
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
    pub fn brush_diameter(&self) -> i32 {
        self.sub_brush_diameter.floor() as i32
    }
    fn generate_circle_mask(&mut self, t_manager: &mut TextureManager) {
        let _ = t_manager.canvas.with_texture_canvas(
            &mut t_manager.open_textures[self.mask_id].texture,
            |c| {
                //create a fake original data
                c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                c.clear();
                let _ = c.filled_pie(
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
        let src = Rect::new(
            0,
            0,
            self.brush_diameter() as u32,
            self.brush_diameter() as u32,
        );

        let (buf_t, mask_t) = indices!(&mut t_manager.open_textures, self.buffer_id, self.mask_id);

        let _ = t_manager
            .canvas
            .with_texture_canvas(&mut buf_t.texture, |c| {
                c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                c.clear();
                let _ = c.copy(&mask_t.texture, None, src);
            });
    }

    // no texture recreation for now
    pub fn resize(&mut self, t_manager: &mut TextureManager, new_size: f32) {
        // self.brush_diameter = new_size;
        // let size = WH::new(self.brush_diameter, self.brush_diameter);
        // t_manager.destroy_open_texture(self.buffer_id);
        // self.buffer_id =
        //     t_manager.init_open_texture(TextureData::new(&t_manager.t_creator, size, None));

        self.sub_brush_diameter = new_size.clamp(1.0, 5000.0);

        // NOTE: just use brush_diameter
        // t_manager.open_textures[self.buffer_id].src =
        //     Some(XYWH::new(0, 0, self.brush_diameter, self.brush_diameter));

        self.update_buffer(t_manager);
    }
    pub fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        input: &InputState,
        stroke_at: XY,
        t_manager: &mut TextureManager,
    ) {
        use ButtonState::*;
        if input.left() == Idle || input.interacting_with != Some(data.targeted_ui_element) {
            return;
        }
        if input.left() == Released {
            data.history.finish_step(t_manager);
            return;
        }

        let radius = self.brush_diameter() / 2;

        if input.left() == Pressed {
            let step = data.history.add_step();
            self.last_stroke_at = stroke_at;
            let bound = stroke_at.to_bound().expand_one(radius);
            let texture_unit_vec = step.get_textures(bound, data.transform, t_manager);
            let buffer = &mut t_manager.open_textures[self.buffer_id];
            let buffer_at = stroke_at.substract_one(radius);
            for unit in texture_unit_vec {
                let texture = &mut t_manager.draw_textures[unit.id];
                self.draw_to_texture(
                    buffer_at.substract(unit.origin),
                    &mut t_manager.canvas,
                    texture,
                    buffer,
                );
            }
            return;
        }
        let step = if let Some(step) = data.history.try_get_target_step() {
            step
        } else {
            return;
        };

        let traveled = stroke_at.distance(self.last_stroke_at);

        let gap_f = max((radius * self.draw_gap_percent) / 100, 1) as f32;
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
                let stroke_box = first.bound_between(last).expand_one(radius);
                if !data.transform.to_bound().is_overlaping(stroke_box) {
                    new_last_stroke = stroke_at;
                    break;
                }

                // getting textures is about 50 times faster than drawing on them
                let texture_unit_vec = step.get_textures(stroke_box, data.transform, t_manager);

                let buffer = &mut t_manager.open_textures[self.buffer_id];

                for st in &strokes {
                    let buffer_at = st.substract_one(radius);
                    for tex in &texture_unit_vec {
                        let texture = &mut t_manager.draw_textures[tex.id];
                        let to_unit_coord = buffer_at.substract(tex.origin);
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

        let dst = buffer_at
            .to_tr_one(self.brush_diameter())
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
}
