use super::ui_element::{ElementType, UIElement};

macro_rules! markup {
    (
        $(
            $variant:ident : $type:ident [ $($child:ident),* ]
        ),* $(,)?
    ) => {
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
                        Id::$variant as usize,
                        vec![ $( Id::$child as usize, )* ]
                    ));
                )*
                vec
            }
        }
    };
}
pub const ID_COUNT: usize = Id::Total as usize;

pub type IdUsize = usize;
// impl Id {
//     pub const fn as_usize(id: Id) -> IdUsize {
//         id as IdUsize
//     }
// }
markup! {
    // main layout
    RootMain:Div[Header,RightWide,RightTools,MainCanvas],
    /**/Header:Div[],
    /**/RightWide:Div[],
    /**/RightTools:Div[IndButtons,BrushButton1,GapButtonBrush,BrushButton2,GapButtonFill],
    /**/MainCanvas:DrawWindow[],

    // tool buttons
    IndButtons:Div[],

    GapButtonBrush:Div[],
    BrushButton1:Button[],

    GapButtonFill:Div[],
    BrushButton2:Button[],

    // for testing
    ForTest1:Div[],
    ForTest2:Div[],
}
