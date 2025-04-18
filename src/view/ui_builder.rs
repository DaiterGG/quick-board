use sdl2::pixels::Color;

use super::ui_element::{ElementType, UIElement};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// unique name for each ui_element
pub enum Id {
    MainDiv,
    MainDivHeader,
    MainDivLeftPanel,
    MainDivRightPanel,
    SoftBorder1,
    LeftBody,
    ButtonTest,
    ForTest1,
    ForTest2,
}

#[derive(Clone, Copy)]
/// unique name for each id_block
pub enum BlockId {
    MainLayout,
    ForTest1,
}

pub struct UIBuilder;
impl UIBuilder {
    pub fn get(id: BlockId) -> UIElement {
        let div: ElementType = ElementType::Div;
        let btn: ElementType = ElementType::Button;
        match id {
            BlockId::MainLayout => UIElement::new(
                div,
                Id::MainDiv,
                vec![
                    // UIElement::new(div, Id::MainDivHeader, Vec::new()),
                    // UIElement::new(
                    //     div,
                    //     Id::MainDivLeftPanel,
                    //     vec![
                    //         UIElement::new(div, Id::SoftBorder1, Vec::new()),
                    //         UIElement::new(div, Id::LeftBody, Vec::new()),
                    //     ],
                    // ),
                    UIElement::new(btn, Id::ButtonTest, Vec::new()),
                ],
            ),
            BlockId::ForTest1 => UIElement::new(div, Id::ForTest1, Vec::new()),
        }
    }
}
