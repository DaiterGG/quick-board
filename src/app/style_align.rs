use super::coords::*;

// pub enum Value {
//     Pixels(i16),
//     Persent(i16),
// }
#[derive(Copy, Clone, Debug)]
pub struct Size {
    hor: i16,
    vert: i16,
    hor_type: TreatAs,
    vert_type: TreatAs,
}
impl Size {
    pub fn new(hor_type: TreatAs, hor: i16, vert_type: TreatAs, vert: i16) -> Self {
        Self {
            hor,
            vert,
            hor_type,
            vert_type,
        }
    }
    pub fn unrap(&self, length: i32, ui_scale: f32) -> WH {
        WH::new(
            unwrap(self.hor, self.hor_type, length, ui_scale),
            unwrap(self.vert, self.vert_type, length, ui_scale),
        )
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
    Horisontal,
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
            direction: Direction::Horisontal,
            side: Side::Start,
            length: Value::new(TreatAs::Percent, 100),
            gap: Value::new(TreatAs::Percent, 0),
        }
    }
}
impl Align {
    pub const fn block(direction: Direction, side: Side, length: Value) -> Align {
        Align::Block {
            direction,
            side,
            length,
            gap: Value::new(TreatAs::Percent, 0),
        }
    }
    pub const fn gap(&mut self, new_gap: Value) -> &mut Self {
        match self {
            Align::Block { gap, .. } => *gap = new_gap,
            Align::Absolute { .. } => {}
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
                let absolute = size.unrap(window_to_fit.w, ui_scale);
                let absolute_window_x = (window_to_fit.w * align_by.x) / 100;
                let absolute_pivot_x = (absolute.w * pivot.x) / 100;
                let new_x = (window_to_fit.x + absolute_window_x) - absolute_pivot_x;

                let absolute_window_y = (window_to_fit.h * align_by.y) / 100;
                let absolute_pivot_y = (absolute.h * pivot.y) / 100;
                let new_y = (window_to_fit.y + absolute_window_y) - absolute_pivot_y;

                // window_to_fit.w = absolute.w;
                // window_to_fit.h = absolute.h;

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
    // TODO: test gap
    gap: &Value,
) -> XYWH {
    let mut block_to_fit = *window_to_split;
    match align_direction {
        Direction::Horisontal => {
            let length = block_length.unwrap(window_to_split.w, ui_scale);
            let gap_length = gap.unwrap(window_to_split.w, ui_scale);
            block_to_fit.w = length;
            window_to_split.w -= length + gap_length;
            match align_side {
                Side::Start => {
                    window_to_split.x += length + gap_length;
                }
                Side::End => {
                    block_to_fit.x += window_to_split.w;
                }
            }
        }
        Direction::Vertical => {
            let length = block_length.unwrap(window_to_split.h, ui_scale);
            let gap_length = gap.unwrap(window_to_split.h, ui_scale);
            block_to_fit.h = length;
            window_to_split.h -= length + gap_length;
            match align_side {
                Side::Start => {
                    window_to_split.y += length + gap_length;
                }
                Side::End => {
                    block_to_fit.y += window_to_split.h;
                }
            }
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
                Direction::Horisontal,
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
