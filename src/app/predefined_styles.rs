use std::collections::HashMap;

use crate::app::{
    style_align::{Size, TreatAs},
    texture_manager::LockedTexId,
};

use super::{
    color_map::ColorTag,
    coords::XY,
    predefined::{ID_COUNT, Id, IdI32},
    slider::Slider,
    style_align::{Align, Direction, Side, Value},
    style_display::{Display, DisplayData},
};

macro_rules! styles {
    (
        $(
            $variant:ident : $type:ident [ $child:expr ]
        ),* $(,)?
    ) => {
        let mut aligns = vec![Align::default(); ID_COUNT];
        let mut displays = vec![None; ID_COUNT];
        let mut sliders_data = HashMap::new();
            $(
                styles!(@process_entry $variant, $type, $child, aligns, displays, sliders_data);
            )*
        (aligns, displays, sliders_data)
    };


    (@process_entry $variant:ident, A, $child:expr, $aligns:ident, $displays:ident, $sliders_data:ident) => {{
        let id = Id::$variant;
        let index = id as usize;
        $aligns[index] = $child;
    }};

    (@process_entry $variant:ident, D, $child:expr, $aligns:ident, $displays:ident, $sliders_data:ident) => {{
        let id = Id::$variant;
        let index = id as usize;
        $displays[index] = Some($child);
    }};

    (@process_entry $variant:ident, Slider, $child:expr, $aligns:ident, $displays:ident, $sliders_data:ident) => {{
        let id = Id::$variant;
        let index = id as usize;
        $sliders_data.insert(index as i32, $child);
    }};
}
pub struct PredefinedStyles;

impl PredefinedStyles {
    pub fn init() -> (Vec<Align>, Vec<Option<Display>>, HashMap<IdI32, Slider>) {
        use Align as A;
        use ColorTag::*;
        use Direction::*;
        use Display as D;
        use DisplayData as Data;
        use Id::*;
        use LockedTexId::*;
        use Side::*;
        use TreatAs::*;
        use Value as V;

        let tool_button_d = D::idle(Data::bg(MainLight))
            .hovered(Data::bg(Sub))
            .pressed(Data::bg(FlashClick));
        let color_slider_handle_d = D::idle(Data::bg(MainLight));
        styles! {
            // main elements block
            // root elements always absolute
            RootMain: A [A::absolute(XY::new(0, 0), XY::new(0, 0), Size::new(Percent, 100, Percent, 100))],

            Header: A [A::block(Vertical, Start, V::new(Pixels, 50))],
            Header: D [D::idle(Data::bg(MainMiddle))],

            RightWide: A [A::block(Horisontal, End, V::new(Pixels, 500))],
            RightWide: D [D::idle(Data::bg(MainMiddle))],

            HSV_H: A [A::block(Vertical, Start, V::new(Percent, 33))],
            HSV_H: Slider [Slider::new(HSV_H_Handle as i32)],

            HSV_H_Handle: A [A::block(Vertical, Start, V::new(Pixels, 30))],
            HSV_H_Handle: D [color_slider_handle_d],

            HSV_S: A [A::block(Vertical, Start, V::new(Percent, 33))],
            HSV_S: Slider [Slider::new(HSV_S_Handle as i32)],

            HSV_S_Handle: A [A::block(Vertical, Start, V::new(Pixels, 30))],
            HSV_S_Handle: D [color_slider_handle_d],

            HSV_V: A [A::block(Vertical, Start, V::new(Percent, 33))],
            HSV_V: Slider [Slider::new(HSV_V_Handle as i32)],

            HSV_V_Handle: A [A::block(Vertical, Start, V::new(Pixels, 30))],
            HSV_V_Handle: D [color_slider_handle_d],

            ColorPicker: A [A::block(Vertical, Start, V::new(Percent, 50))],
            ToolSettings: A [A::block(Vertical, Start, V::new(Percent, 50))],

            ToolSize: A [A::block(Vertical, Start, V::new(Percent, 50))],
            ToolSizeText: A [A::block(Horisontal, Start, V::new(Percent, 50))],

            ToolSizeDrag: A [A::block(Horisontal, Start, V::new(Percent, 50))],
            ToolSizeDrag: D [D::idle(Data::bg(MainDark)).hovered(Data::bg(Sub)).pressed(Data::bg(FlashClick))],


            RightTools: A [A::block(Horisontal, End, V::new(Pixels, 50))],
            RightTools: D [D::idle(Data::bg(MainMiddle))],

            IndButtons: A [A::block(Vertical, Start, V::new(Pixels, 3))],

            BrushButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new(Pixels, 44, Pixels, 44))],
            BrushButton: D [tool_button_d],

            GapButtonBrush: A [A::block(Vertical, Start, V::new(Pixels, 47))],
            GapButtonFill: A [A::block(Vertical, Start, V::new(Pixels, 47))],

            MoveButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new(Pixels, 44, Pixels, 44))],
            MoveButton: D [tool_button_d],


            DrawWindow: D [D::idle(Data::transparent())],
        }
    }
}
