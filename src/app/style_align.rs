use crate::{d, dl};

use super::coords::*;

#[derive(Copy, Clone, Debug)]
pub struct Size {
    hor: i16,
    vert: i16,
    hor_type: SizeTreatAs,
    vert_type: SizeTreatAs,
}
#[derive(Copy, Clone, Debug)]
pub enum SizeTreatAs {
    PercentOfHor,
    PercentOfVert,
    JustPixels,
}
impl Size {
    pub fn new(hor_type: SizeTreatAs, hor: i16, vert_type: SizeTreatAs, vert: i16) -> Self {
        Self {
            hor,
            vert,
            hor_type,
            vert_type,
        }
    }
    pub fn new_one(size_type: SizeTreatAs, size: i16) -> Self {
        Self {
            hor: size,
            vert: size,
            hor_type: size_type,
            vert_type: size_type,
        }
    }

    pub fn unwrap(&self, length: WH, ui_scale: f32) -> WH {
        let mut res = WH::new(0, 0);
        res.w = match self.hor_type {
            SizeTreatAs::PercentOfHor => (length.w * self.hor as i32) / 100,
            SizeTreatAs::PercentOfVert => (length.h * self.hor as i32) / 100,
            SizeTreatAs::JustPixels => (self.hor as f32 * ui_scale) as i32,
        };
        res.h = match self.vert_type {
            SizeTreatAs::PercentOfHor => (length.w * self.vert as i32) / 100,
            SizeTreatAs::PercentOfVert => (length.h * self.vert as i32) / 100,
            SizeTreatAs::JustPixels => (self.vert as f32 * ui_scale) as i32,
        };
        res
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Value {
    value: i16,
    type_of: TreatAs,
}

impl Value {
    pub const fn new(type_of: TreatAs, value: i16) -> Self {
        Self { value, type_of }
    }
    pub fn unwrap(&self, length: i32, ui_scale: f32) -> i32 {
        unwrap(self.value, self.type_of, length, ui_scale)
    }
}
#[derive(Copy, Clone, Debug)]
pub enum TreatAs {
    Percent,
    Pixels,
}
fn unwrap(value: i16, type_of: TreatAs, length: i32, ui_scale: f32) -> i32 {
    match type_of {
        TreatAs::Percent => (length * value as i32) / 100,
        TreatAs::Pixels => (value as f32 * ui_scale) as i32,
    }
}

#[derive(Copy, Clone, Debug)]
/// Start -> Top or left
/// End -> Bottom or Right
pub enum Side {
    Start,
    End,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug)]
pub enum Align {
    Block {
        direction: Direction,
        side: Side,
        length: Value,
        gap: Value,
    },
    Absolute {
        pivot: XY,
        align_by: XY,
        size: Size,
    },
}
impl Default for Align {
    fn default() -> Self {
        Align::Block {
            direction: Direction::Horizontal,
            side: Side::Start,
            length: Value::new(TreatAs::Percent, 100),
            gap: Value::new(TreatAs::Percent, 0),
        }
    }
}
impl Align {
    pub const fn block(direction: Direction, side: Side, length: Value) -> Self {
        Align::Block {
            direction,
            side,
            length,
            gap: Value::new(TreatAs::Percent, 0),
        }
    }
    pub const fn gap(mut self, new_gap: Value) -> Self {
        match &mut self {
            Align::Block { gap, .. } => *gap = new_gap,
            _ => panic!("gap can only be applied to block"),
        }
        self
    }

    pub const fn absolute(pivot: XY, align_by: XY, size: Size) -> Align {
        Self::Absolute {
            pivot,
            align_by,
            size,
        }
    }

    pub fn apply(&self, window_to_fit: &mut XYWH, ui_scale: f32) -> XYWH {
        match &self {
            Align::Block {
                direction,
                side,
                length,
                gap,
            } => {
                //split window into 2 blocks
                split_window(window_to_fit, *length, *side, *direction, ui_scale, gap)
            }
            Align::Absolute {
                pivot,
                align_by,
                size,
            } => {
                let absolute = size.unwrap(window_to_fit.wh(), ui_scale);

                let absolute_window_x = (window_to_fit.w * align_by.x) / 100;
                let absolute_pivot_x = (absolute.w * pivot.x) / 100;
                let new_x = (window_to_fit.x + absolute_window_x) - absolute_pivot_x;

                let absolute_window_y = (window_to_fit.h * align_by.y) / 100;
                let absolute_pivot_y = (absolute.h * pivot.y) / 100;
                let new_y = (window_to_fit.y + absolute_window_y) - absolute_pivot_y;

                XYWH::new(new_x, new_y, absolute.w, absolute.h)
            }
        }
    }
}
/// split XYWH into 2
/// returns the first one
/// mutate window_to_split as second
fn split_window(
    window_to_split: &mut XYWH,
    block_length: Value,
    align_side: Side,
    align_direction: Direction,
    ui_scale: f32,
    gap: &Value,
) -> XYWH {
    let mut block_to_fit = *window_to_split;

    // Destructure direction-dependent components
    let (main_axis_length, main_axis_pos, block_axis_length, block_axis_pos) = match align_direction
    {
        Direction::Horizontal => (
            &mut window_to_split.w,
            &mut window_to_split.x,
            &mut block_to_fit.w,
            &mut block_to_fit.x,
        ),
        Direction::Vertical => (
            &mut window_to_split.h,
            &mut window_to_split.y,
            &mut block_to_fit.h,
            &mut block_to_fit.y,
        ),
    };

    // Calculate lengths using original window size
    let current_length = *main_axis_length;
    let length = block_length.unwrap(current_length, ui_scale);
    let gap_length = gap.unwrap(current_length, ui_scale);

    // Update block and window dimensions
    *block_axis_length = length;
    *main_axis_length -= length + gap_length;

    // Adjust positions based on alignment side
    match align_side {
        Side::Start => {
            *main_axis_pos += length + gap_length;
        }
        Side::End => {
            *block_axis_pos += *main_axis_length;
        }
    }

    block_to_fit
}

#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn it_works_abs(abs in 0i16..8000,
            w_w in 0i32..8000,
            w_h in 0i32..8000,
            w_x in 0i32..8000,
            w_y in 0i32..8000) {
            let style = Align::block(
                Direction::Horizontal,
                Side::Start,
                Value::new(TreatAs::Pixels, abs),
            );
            let mut window = XYWH::new(w_x, w_y, w_w, w_h);
            let result = style.apply(&mut window,1.0);
            assert_eq!(result.x, w_x);
            assert_eq!(result.y, w_y);
            assert_eq!(result.w, abs as i32);
            assert_eq!(result.h, w_h);

            assert_eq!(window.x, w_x + abs as i32);
            assert_eq!(window.y, w_y );
            assert_eq!(window.w, w_w - abs as i32);
            assert_eq!(window.h, w_h);
        }
    }

    proptest! {
        #[test]
        fn it_works_rel(abs in 0i16..100,
            w_w in 0i32..8000,
            w_h in 0i32..8000,
            w_x in 0i32..8000,
            w_y in 0i32..8000) {
            let style = Align::block(
                Direction::Vertical,
                Side::End,
                Value::new(TreatAs::Percent, abs),
            );
            let mut window = XYWH::new(w_x, w_y, w_w, w_h);
            let result = style.apply(&mut window,1.0);
            let abs_to_px = (w_h * abs as i32) / 100;
            assert_eq!(result.x, w_x);
            assert_eq!(result.y, w_y + (w_h - abs_to_px));
            assert_eq!(result.w, w_w);
            assert_eq!(result.h, abs_to_px);

            assert_eq!(window.x, w_x );
            assert_eq!(window.y, w_y );
            assert_eq!(window.w, w_w );
            assert_eq!(window.h, w_h - abs_to_px);
        }
    }
}
