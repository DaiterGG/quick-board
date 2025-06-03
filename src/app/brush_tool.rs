use std::{cmp::*, thread::sleep};

use crate::*;

use super::{
    canvas_manager::*, coords::*, input_state::*, texture_data::TextureData, texture_manager::*,
};
use app::texture_vec::TexId16;
use indices::indices;
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::*,
    rect::*,
    render::*,
    surface::Surface,
    sys::{SDL_ComposeCustomBlendMode, SDL_SetTextureBlendMode},
};

const BUFFER_TEX_SIZE: i32 = 5000;
const BUFFER_TEX_SIZE_I16: i16 = BUFFER_TEX_SIZE as i16;
const ALFA_CIRCLE_TEX_SIZE: i32 = 1000;
/// * `sub_brush_diameter`: 1-5000
/// * private set
/// * pub get sub_brush_diameter as i32
/// *
/// * `alfa`: 1-100
/// * `draw_gap_percent`: 1-100
/// * `alfa_hardness`: 1-100
pub struct Brush {
    sub_brush_diameter: Observed<f32>,
    pub alfa: Observed<i32>,
    pub draw_gap_percent: Observed<i32>,

    last_stroke_at: XY,
    mask_id: TexId16,

    // for circle mask
    pub alfa_hardness: Observed<i32>,
    alfa_mask_id: TexId16,

    // the primary reason why there is a buffer, is to buffer the "scale down" operation
    // of potentially big mask
    buffer_id: TexId16,
}
impl Brush {
    // pub fn enable(&self, data: &mut CanvasData) {
    //     data.update_cursor = true;
    // }
    pub fn new(t_manager: &mut TextureManager) -> Self {
        let texs = &mut t_manager.textures;
        let mask_id = texs.init_texture(TextureData::new(
            &t_manager.t_creator,
            WH::new(BUFFER_TEX_SIZE, BUFFER_TEX_SIZE),
            None,
            Some(TextureAccess::Target),
        ));
        texs.get_mut(mask_id)
            .texture
            .set_blend_mode(BlendMode::Blend);

        let alfa_data = TextureData::new(
            &t_manager.t_creator,
            WH::new(ALFA_CIRCLE_TEX_SIZE, ALFA_CIRCLE_TEX_SIZE),
            None,
            Some(TextureAccess::Target),
        );
        unsafe {
            let custom = SDL_ComposeCustomBlendMode(
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_ADD,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_DST_ALPHA,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ZERO,
                sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_ADD,
            );
            SDL_SetTextureBlendMode(alfa_data.texture.raw(), custom);
        }
        let alfa_mask_id = texs.init_texture(alfa_data);

        let buffer_id = texs.init_texture(TextureData::new(
            &t_manager.t_creator,
            WH::new(BUFFER_TEX_SIZE, BUFFER_TEX_SIZE),
            None,
            Some(TextureAccess::Target),
        ));
        texs.get_mut(buffer_id)
            .texture
            .set_blend_mode(BlendMode::Blend);

        let mut s = Self {
            buffer_id,
            alfa_mask_id,
            sub_brush_diameter: Observed::new(50.0, Box::new(Action::ToolSizeObserve)),
            draw_gap_percent: Observed::new(10, Box::new(Action::BrushDensityObserve)),
            alfa_hardness: Observed::new(1, Box::new(Action::BrushHardnessObserve)),
            last_stroke_at: XY::new(0, 0),
            mask_id,
            alfa: Observed::new(100, Box::new(Action::BrushAlfaObserve)),
        };
        s.generate_circle_mask(t_manager);
        s.generate_circle_alfa_mask(t_manager);
        s.update_buffer(t_manager);
        s
    }
    pub fn brush_diameter(&self) -> i32 {
        self.sub_brush_diameter.get().floor() as i32
    }
    pub fn generate_circle_alfa_mask(&mut self, t_manager: &mut TextureManager) {
        let rad = (ALFA_CIRCLE_TEX_SIZE as f32 / 2.0) as u32;
        let rad_f = ALFA_CIRCLE_TEX_SIZE as f32 / 2.0;
        let mut surface = Surface::new(rad, rad, PixelFormatEnum::RGBA8888).unwrap();

        let hardness = self.alfa_hardness.get().clamp(0, 100) as f32;
        let solid_radius = (hardness / 100.0) * rad_f;

        let solid_radius_sq = solid_radius * solid_radius;
        let radius_sq = rad_f * rad_f;

        surface.with_lock_mut(|pixels| {
            for y in 0..rad {
                for x in 0..rad {
                    let dx = x as f32;
                    let dy = y as f32;
                    let distance_sq = dx * dx + dy * dy;

                    if distance_sq <= solid_radius_sq {
                        pixels[(y * rad + x) as usize * 4] = 255;
                        pixels[(y * rad + x) as usize * 4 + 1] = 255;
                        pixels[(y * rad + x) as usize * 4 + 2] = 255;
                        pixels[(y * rad + x) as usize * 4 + 3] = 255;
                    } else if distance_sq <= radius_sq {
                        // Calculate actual distance only when needed for gradient
                        let distance = distance_sq.sqrt();
                        // Smooth gradient from solid to transparent
                        let t = (distance - solid_radius) / (rad_f - solid_radius);
                        pixels[(y * rad + x) as usize * 4] = (255.0 * (1.0 - t)) as u8;
                        pixels[(y * rad + x) as usize * 4 + 1] = 255;
                        pixels[(y * rad + x) as usize * 4 + 2] = 255;
                        pixels[(y * rad + x) as usize * 4 + 3] = 255;
                    }
                }
            }
        });
        let mut tex = surface.as_texture(&t_manager.t_creator).unwrap();
        tex.set_blend_mode(BlendMode::Blend);

        let alfa_tex = &mut t_manager.textures.get_mut(self.alfa_mask_id).texture;

        t_manager
            .canvas
            .with_texture_canvas(alfa_tex, |c| {
                c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                c.clear();
                let p = None;
                c.copy_ex(&tex, None, Rect::new(0, 0, rad, rad), 0.0, p, true, true)
                    .unwrap();

                let r = Rect::new(rad as i32, 0, rad, rad);
                c.copy_ex(&tex, None, r, 0.0, p, false, true).unwrap();

                let r = Rect::new(rad as i32, rad as i32, rad, rad);
                c.copy(&tex, None, r).unwrap();

                let r = Rect::new(0, rad as i32, rad, rad);
                c.copy_ex(&tex, None, r, 0.0, p, true, false).unwrap();
            })
            .unwrap();

        // t_manager
        //     .canvas
        //     .copy(&t_manager.textures[self.alfa_mask_id].texture, None, None)
        //     .unwrap();
        // t_manager.canvas.present();
        // sleep(std::time::Duration::from_millis(1000));
    }
    pub fn generate_circle_mask(&mut self, t_manager: &mut TextureManager) {
        let tex = &mut t_manager.textures.get_mut(self.mask_id).texture;
        t_manager
            .canvas
            .with_texture_canvas(tex, |c| {
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
                )
                .unwrap();
            })
            .unwrap();
    }

    pub fn update_buffer(&mut self, t_manager: &mut TextureManager) {
        let diameter = self.brush_diameter() as u32;
        let dst = Rect::new(0, 0, diameter, diameter);

        let (buf_t, mask_t, alfa_mask) =
            t_manager
                .textures
                .get_mut_3(self.buffer_id, self.mask_id, self.alfa_mask_id);

        if diameter == 3 && self.alfa_hardness.get() == 100 {
            let color = Color::RGBA(255, 255, 255, 255);
            t_manager
                .canvas
                .with_texture_canvas(&mut buf_t.texture, |c| {
                    c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                    c.clear();
                    c.pixel(1, 0, color).unwrap();
                    c.pixel(0, 1, color).unwrap();
                    c.pixel(1, 1, color).unwrap();
                    c.pixel(2, 1, color).unwrap();
                    c.pixel(1, 2, color).unwrap();
                })
                .unwrap();
            return;
        }
        t_manager
            .canvas
            .with_texture_canvas(&mut buf_t.texture, |c| {
                c.set_draw_color(Color::RGBA(255, 255, 255, 0));
                c.clear();
                c.copy(&mask_t.texture, None, dst).unwrap();
                if self.alfa_hardness.get() != 100 {
                    c.copy(&alfa_mask.texture, None, dst).unwrap();
                }
            })
            .unwrap();
    }

    pub fn mult_size(&mut self, t_manager: &mut TextureManager, by: f32) {
        self.sub_brush_diameter
            .set(self.sub_brush_diameter.get() * by);
        if by > 1.0 {
            self.sub_brush_diameter
                .set(self.sub_brush_diameter.get().ceil());
        } else {
            self.sub_brush_diameter
                .set(self.sub_brush_diameter.get().floor());
        }
        self.sub_brush_diameter
            .set(self.sub_brush_diameter.get().clamp(1.0, 5000.0));
        self.update_buffer(t_manager);
    }
    pub fn add_size(&mut self, t_manager: &mut TextureManager, add: f32) {
        self.sub_brush_diameter
            .set(self.sub_brush_diameter.get() + add);
        self.sub_brush_diameter
            .set(self.sub_brush_diameter.get().clamp(1.0, 5000.0));
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
            let buffer_at = stroke_at.substract_one(radius);
            for unit in texture_unit_vec {
                self.draw_to_texture(
                    buffer_at.substract(unit.origin),
                    data.color.get(),
                    t_manager,
                    unit.id,
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

        let gap_f = max((radius * self.draw_gap_percent.get()) / 100, 1) as f32;
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

                for st in &strokes {
                    let buffer_at = st.substract_one(radius);
                    for tex in &texture_unit_vec {
                        let to_unit_coord = buffer_at.substract(tex.origin);
                        self.draw_to_texture(to_unit_coord, data.color.get(), t_manager, tex.id);
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
        color: Color,
        t_manager: &mut TextureManager,
        unit_id: usize,
    ) {
        let texture = &mut t_manager.draw_textures[unit_id];
        let buffer = &mut t_manager.textures.get_mut(self.buffer_id).texture;

        buffer.set_color_mod(color.r, color.g, color.b);
        let diameter = self.brush_diameter();
        let mut dst = buffer_at
            .to_tr_one(diameter)
            .get_overlap(WH::new_one(DRAW_TEX_SIZE_I32));
        if dst.w == 0 || dst.h == 0 {
            return;
        }

        // FIXME:
        if diameter == 1 {
            dst.x -= 1;
        }

        let src = XYWH::new(-min(buffer_at.x, 0), -min(buffer_at.y, 0), dst.w, dst.h);

        buffer.set_alpha_mod((255 * self.alfa.get() / 100) as u8);
        // let time = std::time::Instant::now();
        t_manager
            .canvas
            .with_texture_canvas(texture, |c| {
                c.copy(&buffer, Some(src.to_rect()), Some(dst.to_rect()))
                    .unwrap();
            })
            .unwrap();
        //if draw_size is 256 - 12,800 us, overlap w 220 h 206 45320
        // print!("{}, ", time.elapsed().as_nanos());
        // println!("overlap {} {} {}", dst.w, dst.h, dst.w * dst.h);
    }
    // NOTE: does not work bc .filled_circle has fixed blend mode
    // pub fn generate_circle_alfa_mask(&mut self, t_manager: &mut TextureManager) {
    //     t_manager
    //         .canvas
    //         .with_texture_canvas(&mut t_manager.textures[self.alfa_mask_id].texture, |c| {
    //             c.set_draw_color(Color::RGBA(255, 255, 255, 0));
    //             c.clear();
    //             let solid = (500 * self.alfa_smoothness) as f32 / 100.0;
    //             let trans = 500.0 - solid;
    //             c.set_blend_mode(BlendMode::None);
    //             d!(trans);
    //             dl!(solid);
    //             for i in 1..=255 {
    //                 let rev_i = 255 - i;
    //                 let rad = (solid + trans * rev_i as f32 / 255.0) as i16;
    //                 d!(rad);
    //                 dl!(i);
    //                 c.filled_circle(
    //                     ALFA_CIRCLE_TEX_SIZE_I16 / 2,
    //                     ALFA_CIRCLE_TEX_SIZE_I16 / 2,
    //                     rad,
    //                     Color::RGBA(255, 255, 255, 1),
    //                 )
    //                 .unwrap();
    //             }
    //             c.filled_circle(
    //                 ALFA_CIRCLE_TEX_SIZE_I16 / 2,
    //                 ALFA_CIRCLE_TEX_SIZE_I16 / 2,
    //                 solid as i16,
    //                 Color::RGBA(255, 0, 0, 255),
    //             )
    //             .unwrap();
    //         })
    //         .unwrap();
    //     t_manager
    //         .canvas
    //         .copy(&t_manager.textures[self.alfa_mask_id].texture, None, None)
    //         .unwrap();
    //     t_manager.canvas.present();
    //     sleep(std::time::Duration::from_millis(1000));
    // }
}
