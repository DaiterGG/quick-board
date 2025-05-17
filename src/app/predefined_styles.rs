use crate::app::texture_manager::LockedTexId;

use super::{
    color_map::ColorTag,
    coords::XY,
    predefined::{ID_COUNT, Id},
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
                $(
                    styles!(@process_entry $variant, $type, $child, aligns, displays);
                )*
        (aligns, displays)
    };


    (@process_entry $variant:ident, A, $child:expr, $aligns:ident, $displays:ident) => {{
        let id = Id::$variant;
        let index = id as usize;
        $aligns[index] = $child;
    }};

    (@process_entry $variant:ident, D, $child:expr, $aligns:ident, $displays:ident) => {{
        let id = Id::$variant;
        let index = id as usize;
        $displays[index] = Some($child);
    }};
}
pub struct PredefinedStyles;

impl PredefinedStyles {
    pub fn new() -> (Vec<Align>, Vec<Option<Display>>) {
        use Align as A;
        use ColorTag::*;
        use Direction::*;
        use Display as D;
        use DisplayData as data;
        use LockedTexId::*;
        use Side::*;
        use Value::*;

        let tool_button_display = D::idle(data::bg(MainLight))
            .hovered(data::bg(Sub))
            .pressed(data::bg(FlashClick));
        styles! {
            // main elements block
            // root elements always absolute
            RootMain: A [A::absolute(XY::new(0, 0), XY::new(0, 0), (Persent(100), Persent(100)))],

            Header: A [A::block(Vertical, Start, Pixels(50))],
            Header: D [D::idle(data::bg(MainMiddle))],

            RightWide: A [A::block(Horisontal, End, Pixels(500))],
            RightWide: D [D::idle(data::bg(MainMiddle))],

            ColorPicker: A [A::block(Vertical, Start, Persent(50))],
            ToolSettings: A [A::block(Vertical, Start, Persent(50))],

            ToolSize: A [A::block(Vertical, Start, Pixels(44))],
            ToolSizeText: A [A::block(Horisontal, Start, Persent(50))],
            ToolSizeDrag: A [A::block(Horisontal, Start, Persent(50))],
            ToolSizeDrag: D [D::idle(data::bg(MainDark)).hovered(data::bg(Sub)).pressed(data::bg(FlashClick))],


            RightTools: A [A::block(Horisontal, End, Pixels(50))],
            RightTools: D [D::idle(data::bg(MainMiddle))],

            IndButtons: A [A::block(Vertical, Start, Pixels(3))],

            BrushButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), (Pixels(44), Pixels(44)))],
            BrushButton: D [tool_button_display],

            GapButtonBrush: A [A::block(Vertical, Start, Pixels(47))],
            GapButtonFill: A [A::block(Vertical, Start, Pixels(47))],

            MoveButton: A [A::absolute(XY::new(50, 0), XY::new(50, 0), (Pixels(44), Pixels(44)))],
            MoveButton: D [tool_button_display],


            DrawWindow: D [D::idle(data::transparent())],

            // test elements
            ForTest1: A [A::block(Horisontal, Start, Persent(40))],
            ForTest2: A [A::block(Horisontal, Start, Persent(100))],
        }
    }
}
