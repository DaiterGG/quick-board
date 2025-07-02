use markup::markup;

use super::ui_element::{ElementType, UIElement};

pub const ID_COUNT: usize = Id::Total as usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Id32(pub u32);

impl Id32 {
    pub fn usize(self) -> usize {
        self.0 as usize
    }
}
impl From<Id> for Id32 {
    fn from(id: Id) -> Self {
        Id32(id as u32)
    }
}
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
                    BrushSize:Div {
                        ToolSizeBlock:Div {
                            ToolSizeNumBlock:Div {
                                ToolSizeNumTxt:Txt,
                            },
                            ToolSizeDrag:Drag,
                        },
                        BrushSizeTxt:Txt,
                    },
                    BrushHardness:Div {
                        BrushHardnessBlock:Div {
                            BrushHardnessNumBlock:Div {
                                BrushHardnessNumTxt:Txt,
                            },
                            BrushHardnessDrag:Drag,
                        },
                        BrushHardnessTxt:Txt,
                    },
                    BrushDensity:Div {
                        BrushDensityBlock:Div {
                            BrushDensityNumBlock:Div {
                                BrushDensityNumTxt:Txt,
                            },
                            BrushDensityDrag:Drag,
                        },
                        BrushDensityTxt:Txt,
                    },
                    BrushAlfa:Div {
                        BrushAlfaBlock:Div {
                            BrushAlfaNumBlock:Div {
                                BrushAlfaNumTxt:Txt,
                            },
                            BrushAlfaDrag:Drag,
                        },
                        BrushAlfaTxt:Txt,
                    },
                    BrushFollow:Div {
                        BrushFollowBlock:Div {
                            BrushFollowNumBlock:Div {
                                BrushFollowNumTxt:Txt,
                            },
                            BrushFollowDrag:Drag,
                        },
                        BrushFollowTxt:Txt,
                    },
                    BrushErase:Div {
                        BrushEraseBlock:Div {
                            BrushEraseCheck:Button {
                                BrushEraseCheckTxt:Txt,
                            },
                        },
                        BrushEraseTxt:Txt,
                    },
                }
            }
        },
        RightTools:Div {
            IndButtons:Div,
            BrushButton:Button { BrushButtonSub:Div },
            GapButton1:Div,
            MoveButton:Button { MoveButtonSub:Div },
            GapButton2:Div,
            SampleButton:Button { SampleButtonSub:Div },
        },
        DrawWindow:DrawWindow
    },

    ForTest1:Div {
        ForTestSub1:Div,
        ForTestSub2:Div,
    },
}}
// NOTE: old macro for reference
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
//                             ElementType::$type,
//                             Id::$variant as i32,
//                             vec![ $( Id::$child as i32, )* ]
//                         ),
//                     )*
//                 ]
//             }
//         }
//     };
// }
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
