use std::collections::HashMap;

use crate::app::{
    border::Border,
    style_align::{Size, TreatAs},
    texture_manager::LockedTexId,
};

use super::{
    color_map::ColorTag,
    coords::XY,
    predefined::{ID_COUNT, Id, IdI32},
    slider::Slider,
    style_align::{Align, Direction, Side, SizeTreatAs, Value},
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
        use SizeTreatAs::*;
        use TreatAs::*;
        use Value as V;

        let thin_b = Border::single_w(BorderDark, 1);
        let tool_button_d = D::idle(Data::bg(MainMiddle).border(thin_b))
            .hovered(Data::bg(Sub))
            .pressed(Data::bg(FlashClick));
        let color_slider_handle_d = D::idle(Data::transparent().border(thin_b));
        let r = D::idle(Data::bg(Red));
        let g = D::idle(Data::bg(Green));
        let b = D::idle(Data::bg(Blue));
        styles! {
            // main elements block
            // root elements always absolute
            RootMain: A [A::absolute(XY::new(0, 0), XY::new(0, 0), Size::new(PercentOfHor, 100, PercentOfVert, 100))],

            Header: A [A::block(Vertical, Start, V::new(Pixels, 50))],
            Header: D [D::idle(Data::bg(MainMiddle).border(Border::all_w(BorderDark, (0,0,1,0))))],

            RightWide: A [A::block(Horizontal, End, V::new(Pixels, 500))],
            RightWide: D [D::idle(Data::bg(MainMiddle))],

            ColorPicker: A [A::block(Vertical, Start, V::new(Percent, 50))],
            PickerHSV: A [A::block(Vertical, Start, V::new(Pixels, 100))],

            HSV_H: A [A::block(Vertical, Start, V::new(Percent, 34))],
            HSV_H: D [D::idle(Data::transparent().locked_texture(RangeHue))],
            HSV_H: Slider [Slider::new(HSV_H_Handle as i32).within()],

            HSV_S: A [A::block(Vertical, Start, V::new(Percent, 50))],
            HSV_S: D [D::idle(Data::transparent().locked_texture(RangeSaturation))],
            HSV_S: Slider [Slider::new(HSV_S_Handle as i32).within()],

            HSV_V: A [A::block(Vertical, Start, V::new(Percent, 100))],
            HSV_V: D [D::idle(Data::transparent().locked_texture(RangeValue))],
            HSV_V: Slider [Slider::new(HSV_V_Handle as i32).within()],

            HSV_H_Handle: A [A::absolute(XY::new(0,50), XY::new(0,50), Size::new(JustPixels, 30, PercentOfVert, 100))],
            HSV_H_Handle: D [color_slider_handle_d],
            HSV_S_Handle: A [A::absolute(XY::new(100,50), XY::new(100,50), Size::new(JustPixels, 30, PercentOfVert, 100))],
            HSV_S_Handle: D [color_slider_handle_d],
            HSV_V_Handle: A [A::absolute(XY::new(100,50), XY::new(100,50), Size::new(JustPixels, 30, PercentOfVert, 100))],
            HSV_V_Handle: D [color_slider_handle_d],

            ToolSettings: A [A::block(Vertical, Start, V::new(Percent, 100))],

            ToolSize: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            ToolSizeText: A [A::block(Horizontal, Start, V::new(Percent, 50))],

            ToolSizeDrag: A [A::block(Horizontal, Start, V::new(Percent, 50))],
            ToolSizeDrag: D [D::idle(Data::bg(MainDark)).hovered(Data::bg(Sub)).pressed(Data::bg(FlashClick))],

            RightTools: A [A::block(Horizontal, End, V::new(Pixels, 50))],
            RightTools: D [D::idle(Data::bg(MainMiddle).border(Border::all_w(BorderDark, (0,1,0,1))))],

            IndButtons: A [A::block(Vertical, Start, V::new(Pixels, 5))],

            BrushButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new_one(PercentOfHor, 85))],
            BrushButton: D [tool_button_d],
            BrushButtonSub: A [A::absolute(XY::new(50, 50), XY::new(50, 50), Size::new_one(PercentOfHor, 90))],
            BrushButtonSub: D [D::idle(Data::transparent().locked_texture(IconBrush))],

            MoveButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new_one(PercentOfHor, 85))],
            MoveButton: D [tool_button_d],
            MoveButtonSub: A [A::absolute(XY::new(50, 50), XY::new(50, 50), Size::new_one(PercentOfHor, 90))],
            MoveButtonSub: D [D::idle(Data::transparent().locked_texture(IconMove))],

            GapButton: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            // GapButtonBrush: A [A::block(Vertical, Start, V::new(Pixels, 47))],

            DrawWindow: D [D::idle(Data::transparent())],
        }
    }
}
