use markup::markup;

use super::ui_element::{ElementType, UIElement};

pub const ID_COUNT: usize = Id::Total as usize;
pub type IdI32 = i32;
// new fancy proc_macro
markup! {{
    RootMain:Div {
        Header:Div,
        RightWide:Div {
            ColorPicker:Div {
                PickerHSV:Div {
                    HSV_H:Slider {
                        HSV_H_Handle:Div
                    },
                    HSV_S:Slider {
                        HSV_S_Handle:Div
                    },
                    HSV_V:Slider {
                        HSV_V_Handle:Div
                    }
                }
            },
            ToolSettings:Div {
                BrushSettings:Div {
                    ToolSize:Div {
                        ToolSizeText:Div,
                        ToolSizeDrag:Drag
                    }
                }
            }
        },
        RightTools:Div {
            IndButtons:Div,
            BrushButton:Button,
            GapButton:Div,
            MoveButton:Button
        },
        DrawWindow:DrawWindow
    }
}}
// macro_rules! markup {
//     (
//         $(
//             $variant:ident : $type:ident [ $($child:ident),* ]
//         ),* $(,)?
//     ) => {
//         #[derive(Copy, Clone, Debug, Eq, PartialEq)]
//         #[allow(non_camel_case_types)]
//         pub enum Id{
//             $($variant,)*
//             Total,
//         }

//         pub struct Predefined;

//         impl Predefined {
//             pub fn init() -> Vec<UIElement> {
//                 vec![
//                     $(
//                         UIElement::new(
//                                 ElementType::$type,
//                                 Id::$variant as i32,
//                                 vec![ $( Id::$child as i32, )* ]
//                         ),
//                     )*
//                 ]
//             }
//         }
//     };
// }
// pub const ID_COUNT: usize = Id::Total as usize;
// pub type IdI32 = i32;
// // impl Id {
// //     pub const fn as_usize(id: Id) -> IdUsize {
// //         id as IdUsize
// //     }
// // }
// markup! {
//     // main layout
//     RootMain:Div[Header,RightWide,RightTools,DrawWindow],
//         Header:Div[],
//         RightWide:Div[ColorPicker,ToolSettings],
//             ColorPicker:Div[],
//                 PickerHSV:Div[HSV_H,HSV_S,HSV_V],
//                     HSV_H:Slider[HSV_H_Handle],
//                         HSV_H_Handle:Div[],
//                     HSV_S:Slider[HSV_S_Handle],
//                         HSV_S_Handle:Div[],
//                     HSV_V:Slider[HSV_V_Handle],
//                         HSV_V_Handle:Div[],
//             ToolSettings:Div[BrushSettings],
//                 BrushSettings:Div[ToolSize],
//                 ToolSize:Div[ToolSizeText,ToolSizeDrag],
//                 ToolSizeText:Div[],
//                 ToolSizeDrag:Drag[],
//         RightTools:Div[IndButtons,BrushButton,GapButtonBrush,MoveButton,GapButtonFill],
//             IndButtons:Div[],
//             GapButtonBrush:Div[],
//             BrushButton:Button[],
//             GapButtonFill:Div[],
//             MoveButton:Button[],
//         DrawWindow:DrawWindow[],
// }
