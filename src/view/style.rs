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
pub enum Style {
    Block {
        direction: AlignDirection,
        side: AlignSide,
        length: AlignValue,
    },
    Absolute {
        pivot: XY,
        align_by: XY,
        size: WH,
    },
    None,
}

#[derive(Copy, Clone)]
pub enum AlignValue {
    Absolute(i32),
    Relative(i32),
}
impl AlignValue {
    pub fn unwrap(&self, length: i32, ui_scale: f32) -> i32 {
        match self {
            AlignValue::Absolute(pixels) => (*pixels as f32 * ui_scale) as i32,
            AlignValue::Relative(percent) => (length * *percent as i32) / 100,
        }
    }
}

#[derive(Copy, Clone)]
/// Start -> Top or left
/// End -> Bottom or Right
pub enum AlignSide {
    Start,
    End,
}

#[derive(Copy, Clone)]
pub enum AlignDirection {
    Horisontal,
    Vertical,
}
impl Style {
    pub const fn block(direction: AlignDirection, side: AlignSide, length: AlignValue) -> Style {
        Style::Block {
            direction,
            side,
            length,
        }
    }

    pub const fn absolute(pivot: XY, align_by: XY, size: WH) -> Style {
        Self::Absolute {
            pivot,
            align_by,
            size,
        }
    }

    pub fn fit_childrens(
        window_to_fit: XYWH,
        childrens: &mut Vec<UIElement>,
        styles: &StyleMap,
        states: &mut States,
    ) {
        //after each child set in place, transform shrinks (for Block),
        //and next child is being applyed to a smaller window
        let mut dynamic_window = window_to_fit;

        for i in 0..childrens.len() {
            let new_transfrom = styles
                .get(childrens[i].get_id())
                .fit_self(&mut dynamic_window, states);
            childrens[i].update_pos(new_transfrom, styles, states);
        }
    }
    fn fit_self(&self, window_to_fit: &mut XYWH, states: &mut States) -> XYWH {
        return match &self {
            Style::Block {
                direction,
                side,
                length,
            } => {
                //split window into 2 blocks
                split_window(window_to_fit, length, *side, *direction, states)
            }
            Style::Absolute {
                pivot,
                align_by,
                size: absolute,
            } => {
                let absolute_window_x = (window_to_fit.w * align_by.x) / 100;
                let absolute_pivot_x = (absolute.w * pivot.x) / 100;
                let new_x = (window_to_fit.x + absolute_window_x) - absolute_pivot_x;

                let absolute_window_y = (window_to_fit.h * align_by.y) / 100;
                let absolute_pivot_y = (absolute.h * pivot.y) / 100;
                let new_y = (window_to_fit.y + absolute_window_y) - absolute_pivot_y;

                XYWH::new(new_x, new_y, absolute.w, absolute.h)
            }
            Style::None => {
                panic!("DisplayType::None")
            }
        };
    }
}
fn split_window(
    window_to_split: &mut XYWH,
    block_length: &AlignValue,
    align_side: AlignSide,
    align_direction: AlignDirection,
    states: &mut States,
) -> XYWH {
    let mut block_to_fit = window_to_split.clone();
    match align_direction {
        AlignDirection::Horisontal => {
            let length = block_length.unwrap(window_to_split.w, states.ui.get_current_ui_scale());
            block_to_fit.w = length;
            window_to_split.w -= length;
            match align_side {
                AlignSide::Start => {
                    window_to_split.x += length;
                }
                AlignSide::End => {
                    block_to_fit.x += window_to_split.w;
                }
            }
        }
        AlignDirection::Vertical => {
            let length = block_length.unwrap(window_to_split.h, states.ui.get_current_ui_scale());
            block_to_fit.h = length;
            window_to_split.h -= length;
            match align_side {
                AlignSide::Start => {
                    window_to_split.y += length;
                }
                AlignSide::End => {
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
            let style = Style::block(
                AlignDirection::Horisontal,
                AlignSide::Start,
                AlignValue::Absolute(abs),
            );
            let mut window = XYWH::new(w_x, w_y, w_w, w_h);
            let result = style.fit_self(&mut window,&mut States::default());
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
            let style = Style::block(
                AlignDirection::Vertical,
                AlignSide::End,
                AlignValue::Relative(abs),
            );
            let mut window = XYWH::new(w_x, w_y, w_w, w_h);
            let result = style.fit_self(&mut window,&mut States::default());
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
