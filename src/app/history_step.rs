use sdl2::{rect::Rect, render::Texture};

use super::{
    coords::*,
    texture_manager::{TextureManager, DRAW_TEX_SIZE},
};

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
    fn get_textures_for_copy(&mut self, transform: XYWH) -> Vec<TextureUnit> {
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

    pub(crate) fn full_draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ui_texture: &mut super::texture_data::TextureData,
        canvas_transform: super::coords::XYWH,
        src: Rect,
        dst: Rect,
    ) {
        todo!()
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
    fn get_textures_for_copy(&mut self, left_id: i32, right_id: i32) -> Vec<TextureUnit> {
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
                origin: XY::new(true_id, row_id),
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
}
}
