use std::i32;

use super::coords::{WH, XY, XYWH};
use super::states::States;
use super::style_map::StyleMap;
use super::ui_element::{UIElement, UIElementTrait};

// potential alternatives
// display_data: Box<dyn Any>, code overhead for casting types
// Style<T: DisplayType>, div need to know store type specific style
// or with duplicate Style {display_absolute, display_block} 👌
// pub struct Style(Type);

#[derive(Copy, Clone)]
pub enum Align {
    Block {
        direction: Direction,
        side: Side,
        length: Value,
    },
    Absolute {
        pivot: XY,
        align_by: XY,
        size: (Value, Value),
    },
    None,
}

#[derive(Copy, Clone)]
pub enum Value {
    Pixels(i32),
    Persent(i32),
}
impl Value {
    pub fn unwrap(&self, length: i32, ui_scale: f32) -> i32 {
        match self {
            Value::Pixels(pixels) => (*pixels as f32 * ui_scale) as i32,
            Value::Persent(percent) => (length * *percent as i32) / 100,
        }
    }
}

#[derive(Copy, Clone)]
/// Start -> Top or left
/// End -> Bottom or Right
pub enum Side {
    Start,
    End,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Horisontal,
    Vertical,
}
impl Align {
    pub const fn block(direction: Direction, side: Side, length: Value) -> Align {
        Align::Block {
            direction,
            side,
            length,
        }
    }

    pub const fn absolute(pivot: XY, align_by: XY, size: (Value, Value)) -> Align {
        Self::Absolute {
            pivot,
            align_by,
            size,
        }
    }

    pub fn align(&self, window_to_fit: &mut XYWH, states: &mut States) -> XYWH {
        return match &self {
            Align::Block {
                direction,
                side,
                length,
            } => {
                //split window into 2 blocks
                split_window(window_to_fit, length, *side, *direction, states)
            }
            Align::Absolute {
                pivot,
                align_by,
                size,
            } => {
                let ui_scale = states.ui.get_current_ui_scale();
                let absolute = WH {
                    w: size.0.unwrap(window_to_fit.w, ui_scale),
                    h: size.1.unwrap(window_to_fit.h, ui_scale),
                };
                let absolute_window_x = (window_to_fit.w * align_by.x) / 100;
                let absolute_pivot_x = (absolute.w * pivot.x) / 100;
                let new_x = (window_to_fit.x + absolute_window_x) - absolute_pivot_x;

                let absolute_window_y = (window_to_fit.h * align_by.y) / 100;
                let absolute_pivot_y = (absolute.h * pivot.y) / 100;
                let new_y = (window_to_fit.y + absolute_window_y) - absolute_pivot_y;

                XYWH::new(new_x, new_y, absolute.w, absolute.h)
            }
            Align::None => {
                panic!("DisplayType::None")
            }
        };
    }
}
fn split_window(
    window_to_split: &mut XYWH,
    block_length: &Value,
    align_side: Side,
    align_direction: Direction,
    states: &mut States,
) -> XYWH {
    let mut block_to_fit = window_to_split.clone();
    match align_direction {
        Direction::Horisontal => {
            let length = block_length.unwrap(window_to_split.w, states.ui.get_current_ui_scale());
            block_to_fit.w = length;
            window_to_split.w -= length;
            match align_side {
                Side::Start => {
                    window_to_split.x += length;
                }
                Side::End => {
                    block_to_fit.x += window_to_split.w;
                }
            }
        }
        Direction::Vertical => {
            let length = block_length.unwrap(window_to_split.h, states.ui.get_current_ui_scale());
            block_to_fit.h = length;
            window_to_split.h -= length;
            match align_side {
                Side::Start => {
                    window_to_split.y += length;
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
        fn it_works_abs(abs in 0i32..8000,
            w_w in 0i32..8000,
            w_h in 0i32..8000,
            w_x in 0i32..8000,
            w_y in 0i32..8000) {
            let style = Align::block(
                Direction::Horisontal,
                Side::Start,
                Value::Pixels(abs),
            );
            let mut window = XYWH::new(w_x, w_y, w_w, w_h);
            let result = style.align(&mut window,&mut States::default());
            assert_eq!(result.x, w_x);
            assert_eq!(result.y, w_y);
            assert_eq!(result.w, abs);
            assert_eq!(result.h, w_h);

            assert_eq!(window.x, w_x + abs);
            assert_eq!(window.y, w_y );
            assert_eq!(window.w, w_w - abs);
            assert_eq!(window.h, w_h);
        }
    }

    proptest! {
        #[test]
        fn it_works_rel(abs in 0i32..100,
            w_w in 0i32..8000,
            w_h in 0i32..8000,
            w_x in 0i32..8000,
            w_y in 0i32..8000) {
            let style = Align::block(
                Direction::Vertical,
                Side::End,
                Value::Persent(abs),
            );
            let mut window = XYWH::new(w_x, w_y, w_w, w_h);
            let result = style.align(&mut window,&mut States::default());
            let abs_to_px = (w_h * abs) / 100;
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
