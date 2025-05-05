use std::cmp::*;

use sdl2::{libc::uint32_t, rect::Rect, render::*, video::Window};

use super::{canvas_manager::CanvasData, coords::*, texture_data::TextureData, texture_manager::*};

// TODO: rename into dynamic texture
pub struct HistoryStep {
    //maintainable onedirectional linked list, to be able
    //to 'walk' on specific layer
    pub next_layer_step: Option<usize>,
    // pub prev_layer_step: Option<usize>,
    rows: Vec<TextureRow>,
    rows_offset: i32,
}
impl HistoryStep {
    pub fn new() -> Self {
        Self {
            // prev_layer_step: None,
            next_layer_step: None,
            rows: Vec::new(),
            rows_offset: 0,
        }
    }
    fn get_textures_for_copy(&self, transform: XYWH) -> Vec<TextureUnit> {
        let left_id = coord_to_id(transform.x);
        let right_id = coord_to_id(transform.x + transform.w);
        let up_id = coord_to_id(transform.y);
        let down_id = coord_to_id(transform.y + transform.h);
        let mut vec = Vec::new();
        for id in up_id..(down_id + 1) {
            let true_id = id + self.rows_offset;

            if true_id < 0 || true_id >= self.rows.len() as i32 {
                continue;
            }
            vec.extend(self.rows[true_id as usize].get_textures_for_copy(left_id, right_id));
        }
        vec
    }
    // will create textures at transforms, if it doesn't exist
    pub fn get_textures(
        &mut self,
        transform: XYWH,
        t_manager: &mut TextureManager,
    ) -> Vec<TextureUnit> {
        let left_id = coord_to_id(transform.x);
        let right_id = coord_to_id(transform.x + transform.w);
        let up_id = coord_to_id(transform.y);
        let down_id = coord_to_id(transform.y + transform.h);
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
            vec.extend(self.rows[true_id as usize].get_textures(left_id, right_id, id, t_manager));
        }
        vec
    }
    // will create texture at coords, if it doesn't exist
    pub fn get_texture(&mut self, pos: XY, t_manager: &mut TextureManager) -> TextureUnit {
        let id_pos = XY::new(coord_to_id(pos.x), coord_to_id(pos.y));
        let mut true_id = id_pos.y + self.rows_offset;
        if true_id < 0 {
            let to_insert = true_id.abs();
            self.rows_offset += to_insert;
            true_id = 0;
            let mut vec = Vec::new();
            for _ in 0..to_insert {
                vec.push(TextureRow::new());
            }
            self.rows.splice(0..0, vec);
        } else if true_id >= self.rows.len() as i32 {
            let to_insert = true_id - self.rows.len() as i32 + 1;
            for _ in 0..to_insert {
                self.rows.push(TextureRow::new());
            }
        }
        self.rows[true_id as usize].get_texture(id_pos.x, id_pos.y, t_manager)
    }

    pub fn full_draw(
        &self,
        canvas: &mut Canvas<Window>,
        t_manager: &mut TextureManager,
        data: &CanvasData,
        src: XYWH,
        dst: XYWH,
    ) {
        let ui_tex = &mut t_manager.open_textures[data.targeted_ui_texture].texture;
        let units = self.get_textures_for_copy(src);
        let to_ui_coord_wh = (DRAW_TEX_SIZE as f32 * data.screen_zoom) as i32;
        for unit in units {
            let tex = &t_manager.draw_textures[unit.id];
            let to_ui_coord = unit
                .origin
                .transform_into(data.screen_zoom, data.screen_pos);

            let overlap = to_ui_coord.get_overlap_const(to_ui_coord_wh, dst);
            let unit_dst = overlap.to_rect();
            let unit_src = Rect::new(
                (max(0, dst.x - to_ui_coord.x) as f32 / data.screen_zoom) as i32,
                (max(0, dst.y - to_ui_coord.y) as f32 / data.screen_zoom) as i32,
                (overlap.w as f32 / data.screen_zoom) as u32,
                (overlap.h as f32 / data.screen_zoom) as u32,
            );

            // println!("{:?},{:?},{:?}", src, to_ui_coord, unit_dst);
            let _ = canvas.with_texture_canvas(ui_tex, |c| {
                let _ = c.copy(tex, unit_src, unit_dst);
                ()
            });
        }
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
    // fn try_get_texture(&mut self, id: i32) -> Option<usize> {
    //     let true_id = id + self.row_offset;
    //     if true_id < 0 {
    //         None
    //     } else if true_id >= self.units.len() as i32 {
    //         None
    //     } else {
    //         self.units[true_id as usize]
    //     }
    // }
    fn get_textures_for_copy(&self, left_id: i32, right_id: i32) -> Vec<TextureUnit> {
        let mut vec = Vec::new();
        for id in left_id..(right_id + 1) {
            let true_id = id + self.row_offset;
            if true_id < 0 || true_id >= self.units.len() as i32 {
                continue;
            }
            if let Some(tex) = self.units[true_id as usize] {
                vec.push(tex);
            }
        }
        vec
    }
    fn get_textures(
        &mut self,
        left_id: i32,
        right_id: i32,
        row_id: i32,
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
                    id: t_manager.init_draw_texture(),
                    origin: XY::new(id_to_coord(id), id_to_coord(row_id)),
                };
                self.units[true_id as usize] = Some(unit);
                vec.push(unit);
            }
        }
        vec
    }
    fn get_texture(&mut self, id: i32, row_id: i32, t_manager: &mut TextureManager) -> TextureUnit {
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
            unit
        } else {
            let unit = TextureUnit {
                id: t_manager.init_draw_texture(),
                origin: XY::new(id_to_coord(id), id_to_coord(row_id)),
            };
            self.units[true_id as usize] = Some(unit);
            unit
        }
    }
}
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
    id * DRAW_TEX_SIZE as i32
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
        assert_eq!(coord_to_id(255), 0);
        assert_eq!(coord_to_id(256), 1);
        assert_eq!(coord_to_id(-256), -1);
        assert_eq!(coord_to_id(-257), -2);
    }
    #[test]
    fn test_id_to_coord() {
        assert_eq!(id_to_coord(0), 0);
        assert_eq!(id_to_coord(1), 256);
        assert_eq!(id_to_coord(-1), -256);
    }
}
