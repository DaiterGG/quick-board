use super::ui_element::{ElementType, UIElement};

macro_rules! markup {
    (
        $(
            $variant:ident : $type:ident [ $($child:ident),* ]
        ),* $(,)?
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum Id{
            $($variant,)*
            Total,
        }

        pub struct Predefined;

        impl Predefined {
            pub fn new() -> Vec<UIElement> {
                let mut vec = Vec::new();
                $(
                    vec.push(UIElement::new(
                        ElementType::$type,
                        Id::$variant as i32,
                        vec![ $( Id::$child as i32, )* ]
                    ));
                )*
                vec
            }
        }
    };
}
pub const ID_COUNT: usize = Id::Total as usize;

pub type IdI32 = i32;
// impl Id {
//     pub const fn as_usize(id: Id) -> IdUsize {
//         id as IdUsize
//     }
// }
markup! {
    // main layout
    RootMain:Div[Header,RightWide,RightTools,DrawWindow],
    /**/Header:Div[],

    /**/RightWide:Div[ColorPicker,ToolSettings],
    /**//**/ColorPicker:Div[],
    /**//**/ToolSettings:Div[BrushSettings],

    /**/RightTools:Div[IndButtons,BrushButton,GapButtonBrush,MoveButton,GapButtonFill],
    /**/DrawWindow:DrawWindow[],

    // tool settings
    BrushSettings:Div[ToolSize],
    ToolSize:Div[ToolSizeText,ToolSizeDrag],
    ToolSizeText:Div[],
    ToolSizeDrag:Drag[],

    // tool buttons
    IndButtons:Div[],
    GapButtonBrush:Div[],
    BrushButton:Button[],
    GapButtonFill:Div[],
    MoveButton:Button[],

    // for testing
    ForTest1:Div[],
    ForTest2:Div[],
}
