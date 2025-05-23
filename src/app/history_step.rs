use num::Float;
use num::cast::ToPrimitive;
use std::cmp::*;

use sdl2::rect::{FRect, Rect};

use crate::*;

use super::{canvas_manager::CanvasData, coords::*, texture_data::TextureData, texture_manager::*};

// TODO: rename into dynamic texture
pub struct HistoryStep {
    //maintainable onedirectional linked list, to be able
    //to 'walk' on specific layer
    pub next_layer_step: Option<usize>,
    pub is_static: bool,
    // pub prev_layer_step: Option<usize>,
    rows: Vec<TextureRow>,
    rows_offset: i32,

    // keep a copy of units to draw them in a loop
    flat_copy: Vec<TextureUnit>,
}
impl HistoryStep {
    pub fn new() -> Self {
        Self {
            is_static: false,
            // prev_layer_step: None,
            next_layer_step: None,
            rows: Vec::new(),
            rows_offset: 0,
            flat_copy: Vec::new(),
        }
    }
    fn get_textures_for_copy(&self, transform: XYWH) -> Vec<TextureUnit> {
        let mut vec = Vec::new();
        for unit in &self.flat_copy {
            if unit.origin.x >= transform.x - DRAW_TEX_SIZE_I32
                && unit.origin.x <= transform.x + transform.w
                && unit.origin.y >= transform.y - DRAW_TEX_SIZE_I32
                && unit.origin.y <= transform.y + transform.h
            {
                vec.push(*unit);
            }
        }
        vec
    }
    pub fn get_textures(
        &mut self,
        bound: XXYY,
        canvas_transform: XYWH,
        t_manager: &mut TextureManager,
    ) -> Vec<TextureUnit> {
        let canvas_bound = canvas_transform.to_bound();
        let left_id = coord_to_id(max(bound.xa, canvas_bound.xa));
        let right_id = coord_to_id(min(bound.xb, canvas_bound.xb));
        let up_id = coord_to_id(max(bound.ya, canvas_bound.ya));
        let down_id = coord_to_id(min(bound.yb, canvas_bound.yb));
        let mut vec = Vec::new();

        for id in up_id..(down_id + 1) {
            let mut true_id = id + self.rows_offset;

            if true_id < 0 {
                let to_insert = true_id.abs();
                self.rows_offset += to_insert;
                true_id = 0;
                let mut row_vec = Vec::new();
                for _ in 0..to_insert {
                    row_vec.push(TextureRow::new());
                }
                self.rows.splice(0..0, row_vec);
            } else if true_id >= self.rows.len() as i32 {
                let to_insert = true_id - self.rows.len() as i32 + 1;
                for _ in 0..to_insert {
                    self.rows.push(TextureRow::new());
                }
            }
            let row = self.rows[true_id as usize].get_textures(
                left_id,
                right_id,
                id,
                &mut self.flat_copy,
                t_manager,
            );
            vec.extend(row);
        }
        vec
    }
    //  NOTE: return single texture unit
    // pub fn get_texture(&mut self, pos: XY, t_manager: &mut TextureManager) -> TextureUnit {
    //     let id_pos = XY::new(coord_to_id(pos.x), coord_to_id(pos.y));
    //     let mut true_id = id_pos.y + self.rows_offset;
    //     if true_id < 0 {
    //         let to_insert = true_id.abs();
    //         self.rows_offset += to_insert;
    //         true_id = 0;
    //         let mut vec = Vec::new();
    //         for _ in 0..to_insert {
    //             vec.push(TextureRow::new());
    //         }
    //         self.rows.splice(0..0, vec);
    //     } else if true_id >= self.rows.len() as i32 {
    //         let to_insert = true_id - self.rows.len() as i32 + 1;
    //         for _ in 0..to_insert {
    //             self.rows.push(TextureRow::new());
    //         }
    //     }
    //     self.rows[true_id as usize].get_texture(id_pos.x, id_pos.y, t_manager)
    // }

    /// duplicated function for when canvas is really large
    pub fn full_draw_double(
        &self,
        t_manager: &mut TextureManager,
        data: &CanvasData,
        src: XYWH,
        dst: XYWH,
    ) {
        let ui_tex = &mut t_manager.open_textures[data.targeted_ui_texture].texture;
        let units = self.get_textures_for_copy(src);

        let zoom_64 = data.screen_zoom as f64;
        let to_ui_coord_wh = DRAW_TEX_SIZE as f64 * zoom_64;

        let dst_x = dst.x as f64;
        let dst_y = dst.y as f64;
        let dst_w = dst.w as f64;
        let dst_h = dst.h as f64;

        for i in 0..units.len() {
            let unit = units[i];
            let tex = &t_manager.draw_textures[unit.id];

            let to_ui_coord_x = (unit.origin.x as f64 * zoom_64) + data.screen_pos.x as f64;
            let to_ui_coord_y = (unit.origin.y as f64 * zoom_64) + data.screen_pos.y as f64;

            let overlap_x = f64::max(to_ui_coord_x, dst_x);
            let overlap_y = f64::max(to_ui_coord_y, dst_y);

            let overlap_w = f64::min(to_ui_coord_x + to_ui_coord_wh, dst_x + dst_w) - overlap_x;
            let overlap_h = f64::min(to_ui_coord_y + to_ui_coord_wh, dst_y + dst_h) - overlap_y;

            let unit_src = Rect::new(
                (f64::max(0.0, dst_x - to_ui_coord_x) / zoom_64) as i32,
                (f64::max(0.0, dst_y - to_ui_coord_y) / zoom_64) as i32,
                (overlap_w / zoom_64).round() as u32,
                (overlap_h / zoom_64).round() as u32,
            );

            let unit_dst = FRect::new(
                overlap_x as f32,
                overlap_y as f32,
                overlap_w as f32,
                overlap_h as f32,
            );
            let _ = t_manager.canvas.with_texture_canvas(ui_tex, |c| {
                let _ = c.copy_f(tex, unit_src, unit_dst);
            });
        }
    }

    pub fn full_draw(
        &self,
        t_manager: &mut TextureManager,
        data: &CanvasData,
        src: XYWH,
        dst: XYWH,
    ) {
        let ui_tex = &mut t_manager.open_textures[data.targeted_ui_texture].texture;
        let units = self.get_textures_for_copy(src);

        let to_ui_coord_wh = DRAW_TEX_SIZE as f32 * data.screen_zoom;

        // NOTE: inlined to convert middle state into f32 to eliminate rounding errors
        let dst_x = dst.x as f32;
        let dst_y = dst.y as f32;
        let dst_w = dst.w as f32;
        let dst_h = dst.h as f32;

        for i in 0..units.len() {
            let unit = units[i];
            let tex = &t_manager.draw_textures[unit.id];

            let to_ui_coord_x =
                (unit.origin.x as f32 * data.screen_zoom) + data.screen_pos.x as f32;
            let to_ui_coord_y =
                (unit.origin.y as f32 * data.screen_zoom) + data.screen_pos.y as f32;

            let overlap_x = f32::max(to_ui_coord_x, dst_x);
            let overlap_y = f32::max(to_ui_coord_y, dst_y);

            let overlap_w = f32::min(to_ui_coord_x + to_ui_coord_wh, dst_x + dst_w) - overlap_x;
            let overlap_h = f32::min(to_ui_coord_y + to_ui_coord_wh, dst_y + dst_h) - overlap_y;

            let w_f = overlap_w / data.screen_zoom;
            let h_f = overlap_h / data.screen_zoom;
            let x = (f32::max(0.0, dst_x - to_ui_coord_x) / data.screen_zoom) as i32;
            let y = (f32::max(0.0, dst_y - to_ui_coord_y) / data.screen_zoom) as i32;
            let w = w_f.ceil();
            let h = h_f.ceil();
            let unit_src = Rect::new(x, y, w as u32, h as u32);

            let unit_dst = if data.screen_zoom > 1.0 {
                let dif_x = (w - w_f) * data.screen_zoom;
                let dif_y = (h - h_f) * data.screen_zoom;
                FRect::new(overlap_x, overlap_y, overlap_w + dif_x, overlap_h + dif_y)
            } else {
                FRect::new(overlap_x, overlap_y, overlap_w, overlap_h)
            };

            t_manager
                .canvas
                .with_texture_canvas(ui_tex, |c| c.copy_f(tex, unit_src, unit_dst).unwrap())
                .unwrap();
        }
    }
    pub fn make_static(&mut self, t_manager: &mut TextureManager) {
        if self.is_static {
            return;
        }
        for i in 0..self.flat_copy.len() {
            let unit = self.flat_copy[i];
            t_manager.make_static(unit.id);
        }
        self.is_static = true;
    }
}
/// * `textures`: Some(id) points to a valid, "used" texture
/// * `row_offset`: positive number that shifts the row index
struct TextureRow {
    units: Vec<Option<TextureUnit>>,
    row_offset: i32,
}
impl TextureRow {
    fn new() -> Self {
        Self {
            units: Vec::new(),
            row_offset: 0,
        }
    }
    fn get_textures(
        &mut self,
        left_id: i32,
        right_id: i32,
        row_id: i32,
        flat: &mut Vec<TextureUnit>,
        t_manager: &mut TextureManager,
    ) -> Vec<TextureUnit> {
        let mut vec = Vec::new();
        for id in left_id..(right_id + 1) {
            let mut true_id = id + self.row_offset;
            if true_id < 0 {
                let to_insert = true_id.abs();
                self.row_offset += to_insert;
                let mut vec = Vec::new();
                for _ in 0..to_insert {
                    vec.push(None);
                }
                self.units.splice(0..0, vec);
                true_id = 0;
            } else if true_id >= self.units.len() as i32 {
                let to_insert = true_id - self.units.len() as i32 + 1;
                for _ in 0..to_insert {
                    self.units.push(None);
                }
            }
            if let Some(unit) = self.units[true_id as usize] {
                vec.push(unit);
            } else {
                let unit = TextureUnit {
                    id: t_manager.init_target_texture(),
                    origin: XY::new(id_to_coord(id), id_to_coord(row_id)),
                };
                // println!("init id: {}, {}", id_to_coord(id), id_to_coord(row_id));
                self.units[true_id as usize] = Some(unit);
                flat.push(unit);

                vec.push(unit);
            }
        }
        vec
    }
    //  NOTE: return single texture unit
    // fn get_texture(&mut self, id: i32, row_id: i32, t_manager: &mut TextureManager) -> TextureUnit {
    //     let mut true_id = id + self.row_offset;
    //     if true_id < 0 {
    //         let to_insert = true_id.abs();
    //         self.row_offset += to_insert;
    //         let mut vec = Vec::new();
    //         for _ in 0..to_insert {
    //             vec.push(None);
    //         }
    //         self.units.splice(0..0, vec);
    //         true_id = 0;
    //     } else if true_id >= self.units.len() as i32 {
    //         let to_insert = true_id - self.units.len() as i32 + 1;
    //         for _ in 0..to_insert {
    //             self.units.push(None);
    //         }
    //     }
    //     if let Some(unit) = self.units[true_id as usize] {
    //         unit
    //     } else {
    //         let unit = TextureUnit {
    //             id: t_manager.init_draw_texture(),
    //             origin: XY::new(id_to_coord(id), id_to_coord(row_id)),
    //         };
    //         self.units[true_id as usize] = Some(unit);
    //         unit
    //     }
    // }
}

//DRAW_TEX_SIZE = 256
// 0 -> 0, 255 -> 0, 256 -> 1
// -256 -> -1, -257 -> -2
fn coord_to_id(coord: i32) -> i32 {
    f32::floor(coord as f32 / DRAW_TEX_SIZE as f32) as i32
    // off by 1 at -257
    // coord / DRAW_TEX_SIZE as i32
}
// 0 -> 0, 1 -> 256
// -1 -> -256
fn id_to_coord(id: i32) -> i32 {
    id * DRAW_TEX_SIZE_I32
}
#[derive(Copy, Clone)]
pub struct TextureUnit {
    pub id: usize,
    pub origin: XY,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coord_to_id() {
        assert_eq!(coord_to_id(0), 0);
        assert_eq!(coord_to_id(DRAW_TEX_SIZE_I32 - 1), 0);
        assert_eq!(coord_to_id(DRAW_TEX_SIZE_I32), 1);
        assert_eq!(coord_to_id(-DRAW_TEX_SIZE_I32), -1);
        assert_eq!(coord_to_id(-DRAW_TEX_SIZE_I32 - 1), -2);
    }
    #[test]
    fn test_id_to_coord() {
        assert_eq!(id_to_coord(0), 0);
        assert_eq!(id_to_coord(1), DRAW_TEX_SIZE_I32);
        assert_eq!(id_to_coord(-1), -DRAW_TEX_SIZE_I32);
    }
}
