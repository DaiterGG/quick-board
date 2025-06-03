use std::{any::Any, collections::HashMap};

use super::{
    action_pump::*,
    align_vec::AlignVec,
    border::Border,
    color_map::ColorTag,
    coords::XY,
    display_vec::DisplayVec,
    predefined::{ID_COUNT, Id, Id32},
    slider::Slider,
    style_align::{Align, Direction, Side, Size, SizeTreatAs, TreatAs, Value},
    style_display::{Display, DisplayData, DisplayState},
    texture_vec::TexId,
    txt::Txt,
    ui_map::ElemDataMap,
};

macro_rules! styles {
    (
        $(
            $variant:ident : $type:ident [ $child:expr ]
        ),* $(,)?
    ) => {
        let mut aligns = vec![Some(Align::default()); ID_COUNT];
        let mut displays = vec![None; ID_COUNT];
        let mut data: HashMap<Id32, Box<dyn Any>> = HashMap::new();
            $(
                styles!(@process_entry $variant, $type, $child, aligns, displays, data);
            )*
        let displays = DisplayVec::new(displays);
        let aligns = AlignVec::new(aligns);
        (aligns, displays,data)
    };


    (@process_entry $variant:ident, A, $child:expr, $aligns:ident, $displays:ident, $data:ident) => {{
        $aligns[Id::$variant as usize] = Some($child);
    }};

    (@process_entry $variant:ident, D, $child:expr, $aligns:ident, $displays:ident, $data:ident) => {{
        $displays[Id::$variant as usize] = Some($child);
    }};

    (@process_entry $variant:ident, Subscribe, $child:expr, $aligns:ident, $displays:ident, $data:ident) => {{
        ActionPump::subscribe_to_callback(Id::$variant.into(), $child);
    }};

    //txt is auto subscribed to ui_scale changes
    (@process_entry $variant:ident, Txt, $child:expr, $aligns:ident, $displays:ident, $data:ident) => {{
        ActionPump::subscribe_to_callback(Id::$variant.into(), Callback::TxtScale);
        $data.insert(Id::$variant.into(), Box::new($child));
    }};

    //catch all other &type's it put element specific data
    (@process_entry $variant:ident, $type:ident, $child:expr, $aligns:ident, $displays:ident, $data:ident) => {{
        $data.insert(Id::$variant.into(), Box::new($child));
    }};
}
pub struct PredefinedStyles;

impl PredefinedStyles {
    pub fn init() -> (AlignVec, DisplayVec, ElemDataMap) {
        use Align as A;
        use ColorTag::*;
        use Direction::*;
        use Display as D;
        use DisplayData as Data;
        use DisplayState::*;
        use Id::*;
        use Side::*;
        use SizeTreatAs::*;
        use TexId::*;
        use TreatAs::*;
        use Value as V;

        let thin_b = Border::single_w(BorderDark, 1);
        let tool_button_d = D::idle(Data::new(MainMiddle).border(thin_b))
            .hovered(Data::new(Sub))
            .pressed(Data::new(FlashClick));
        let color_slider_handle_d = D::idle(Data::transparent().border(thin_b));
        let r = D::idle(Data::new(Red));
        let g = D::idle(Data::new(Green));
        let b = D::idle(Data::new(Blue));
        styles! {
            // main elements block
            // root elements always absolute
            RootMain: A [A::absolute(XY::new(0, 0), XY::new(0, 0), Size::new(PercentOfHor, 100, PercentOfVert, 100))],

            Header: A [A::block(Vertical, Start, V::new(Pixels, 50))],
            Header: D [D::idle(Data::new(MainMiddle).border(Border::all_w(BorderDark, (0,0,1,0))))],

            RightWide: A [A::block(Horizontal, End, V::new(Pixels, 500))],
            RightWide: D [D::idle(Data::new(MainMiddle))],

            ColorPicker: A [A::block(Vertical, Start, V::new(Percent, 10))],
            PickerHSV: A [A::block(Vertical, Start, V::new(Pixels, 100))],

            HSV_H: A [A::block(Vertical, Start, V::new(Percent, 34))],
            HSV_H: D [D::idle(Data::transparent().with_tex(RangeHue))],
            HSV_H: Slider [Slider::new(HSV_H_Handle.into()).within()],
            HSV_H: Subscribe [Callback::HSVHue],

            HSV_S: A [A::block(Vertical, Start, V::new(Percent, 50))],
            HSV_S: D [D::idle(Data::transparent().with_tex(RangeSaturation))],
            HSV_S: Slider [Slider::new(HSV_S_Handle.into()).within()],
            HSV_S: Subscribe [Callback::HSVSaturation],

            HSV_V: A [A::block(Vertical, Start, V::new(Percent, 100))],
            HSV_V: D [D::idle(Data::transparent().with_tex(RangeValue))],
            HSV_V: Slider [Slider::new(HSV_V_Handle.into()).within()],
            HSV_V: Subscribe [Callback::HSVValue],

            HSV_H_Handle: A [A::absolute(XY::new(0,50), XY::new(0,50), Size::new(JustPixels, 30, PercentOfVert, 100))],
            HSV_H_Handle: D [color_slider_handle_d],
            HSV_S_Handle: A [A::absolute(XY::new(100,50), XY::new(100,50), Size::new(JustPixels, 30, PercentOfVert, 100))],
            HSV_S_Handle: D [color_slider_handle_d],
            HSV_V_Handle: A [A::absolute(XY::new(100,50), XY::new(100,50), Size::new(JustPixels, 30, PercentOfVert, 100))],
            HSV_V_Handle: D [color_slider_handle_d],

            ToolSettings: A [A::block(Vertical, Start, V::new(Percent, 100))],
            // ToolSettings: D [D::idle(Data::new(MainMiddle).border(Border::all_w(BorderDark, (1,0,0,0))))],


            BrushSize: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            ToolSizeBlock: A [A::block(Horizontal, End, V::new(Percent, 50))],

            ToolSizeNumBlock: A [A::block(Horizontal, Start, V::new(Pixels, 90))],
            ToolSizeNumBlock: D [D::idle(Data::new(MainDark).border(Border::all_w(BorderDark, (0,1,0,0))))],

            ToolSizeNumTxt: A [A::absolute(XY::new(50, 40), XY::new(50, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            ToolSizeNumTxt: D [D::idle(Data::transparent())],
            ToolSizeNumTxt: Txt [Txt::new("".to_string(), 30, vec![Idle])],
            ToolSizeNumTxt: Subscribe [Callback::ToolSizeTxt],

            ToolSizeDrag: A [A::block(Horizontal, Start, V::new(Pixels, 30))],
            ToolSizeDrag: D [D::idle(Data::new(MainDark)).hovered(Data::new(Sub)).pressed(Data::new(FlashClick))],

            BrushSizeTxt: A [A::absolute(XY::new(100, 40), XY::new(95, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushSizeTxt: D [D::idle(Data::transparent())],
            BrushSizeTxt: Txt [Txt::new("Brush Size".to_string(), 30, vec![Idle])],


            BrushHardness: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            BrushHardnessBlock: A [A::block(Horizontal, End, V::new(Percent, 50))],

            BrushHardnessNumBlock: A [A::block(Horizontal, Start, V::new(Pixels, 90))],
            BrushHardnessNumBlock: D [D::idle(Data::new(MainDark).border(Border::all_w(BorderDark, (0,1,0,0))))],

            BrushHardnessNumTxt: A [A::absolute(XY::new(50, 40), XY::new(50, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushHardnessNumTxt: D [D::idle(Data::transparent())],
            BrushHardnessNumTxt: Txt [Txt::new("".to_string(), 30, vec![Idle])],
            BrushHardnessNumTxt: Subscribe [Callback::BrushHardnessTxt],

            BrushHardnessDrag: A [A::block(Horizontal, Start, V::new(Pixels, 30))],
            BrushHardnessDrag: D [D::idle(Data::new(MainDark)).hovered(Data::new(Sub)).pressed(Data::new(FlashClick))],
            BrushHardnessTxt: A [A::absolute(XY::new(100, 40), XY::new(95, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushHardnessTxt: D [D::idle(Data::transparent())],
            BrushHardnessTxt: Txt [Txt::new("Brush Hardness".to_string(), 30, vec![Idle])],


            BrushDensity: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            BrushDensityBlock: A [A::block(Horizontal, End, V::new(Percent, 50))],

            BrushDensityNumBlock: A [A::block(Horizontal, Start, V::new(Pixels, 90))],
            BrushDensityNumBlock: D [D::idle(Data::new(MainDark).border(Border::all_w(BorderDark, (0,1,0,0))))],

            BrushDensityNumTxt: A [A::absolute(XY::new(50, 40), XY::new(50, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushDensityNumTxt: D [D::idle(Data::transparent())],
            BrushDensityNumTxt: Txt [Txt::new("".to_string(), 30, vec![Idle])],
            BrushDensityNumTxt: Subscribe [Callback::BrushDensityTxt],

            BrushDensityDrag: A [A::block(Horizontal, Start, V::new(Pixels, 30))],
            BrushDensityDrag: D [D::idle(Data::new(MainDark)).hovered(Data::new(Sub)).pressed(Data::new(FlashClick))],
            BrushDensityTxt: A [A::absolute(XY::new(100, 40), XY::new(95, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushDensityTxt: D [D::idle(Data::transparent())],
            BrushDensityTxt: Txt [Txt::new("Brush Density".to_string(), 30, vec![Idle])],


            BrushAlfa: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            BrushAlfaBlock: A [A::block(Horizontal, End, V::new(Percent, 50))],

            BrushAlfaNumBlock: A [A::block(Horizontal, Start, V::new(Pixels, 90))],
            BrushAlfaNumBlock: D [D::idle(Data::new(MainDark).border(Border::all_w(BorderDark, (0,1,0,0))))],

            BrushAlfaNumTxt: A [A::absolute(XY::new(50, 40), XY::new(50, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushAlfaNumTxt: D [D::idle(Data::transparent())],
            BrushAlfaNumTxt: Txt [Txt::new("".to_string(), 30, vec![Idle])],
            BrushAlfaNumTxt: Subscribe [Callback::BrushAlfaTxt],

            BrushAlfaDrag: A [A::block(Horizontal, Start, V::new(Pixels, 30))],
            BrushAlfaDrag: D [D::idle(Data::new(MainDark)).hovered(Data::new(Sub)).pressed(Data::new(FlashClick))],
            BrushAlfaTxt: A [A::absolute(XY::new(100, 40), XY::new(95, 50), Size::new(JustPixels, -1, PercentOfVert, 100))],
            BrushAlfaTxt: D [D::idle(Data::transparent())],
            BrushAlfaTxt: Txt [Txt::new("Brush Alfa".to_string(), 30, vec![Idle])],

            RightTools: A [A::block(Horizontal, End, V::new(Pixels, 50))],
            RightTools: D [D::idle(Data::new(MainMiddle).border(Border::all_w(BorderDark, (0,1,0,1))))],

            IndButtons: A [A::block(Vertical, Start, V::new(Pixels, 5))],

            BrushButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new_one(PercentOfHor, 85))],
            BrushButton: D [tool_button_d],
            BrushButtonSub: A [A::absolute(XY::new(50, 50), XY::new(50, 50), Size::new_one(PercentOfHor, 90))],
            BrushButtonSub: D [D::idle(Data::transparent().with_tex(IconBrush))],

            MoveButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new_one(PercentOfHor, 85))],
            MoveButton: D [tool_button_d],
            MoveButtonSub: A [A::absolute(XY::new(50, 50), XY::new(50, 50), Size::new_one(PercentOfHor, 90))],
            MoveButtonSub: D [D::idle(Data::transparent().with_tex(IconMove))],

            GapButton1: A [A::block(Vertical, Start, V::new(Pixels, 44))],
            GapButton2: A [A::block(Vertical, Start, V::new(Pixels, 44))],

            SampleButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), Size::new_one(PercentOfHor, 85))],
            SampleButton: D [tool_button_d],
            SampleButtonSub: A [A::absolute(XY::new(50, 50), XY::new(50, 50), Size::new_one(PercentOfHor, 90))],
            SampleButtonSub: D [D::idle(Data::transparent().with_tex(IconSample))],
            // GapButtonBrush: A [A::block(Vertical, Start, V::new(Pixels, 47))],

            DrawWindow: D [D::idle(Data::transparent())],
        }
    }
}
