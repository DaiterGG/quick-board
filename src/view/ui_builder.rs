use sdl2::pixels::Color;

use super::ui_element::{ElementType, UIElement};

#[derive(Clone, Copy)]
/// unique name for each ui_element
pub enum Id {
    MainDiv,
    MainDivHeader,
    MainDivLeftPanel,
    MainDivRightPanel,
    SoftBorder1,
    LeftBody,
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
        match id {
            BlockId::MainLayout => UIElement::new(
                Id::MainDiv,
                div,
                vec![
                    UIElement::new(Id::MainDivHeader, div, Vec::new()),
                    UIElement::new(
                        Id::MainDivLeftPanel,
                        div,
                        vec![
                            UIElement::new(Id::SoftBorder1, div, Vec::new()),
                            UIElement::new(Id::LeftBody, div, Vec::new()),
                        ],
                    ),
                    // UIElement::new(Id::MainDivRightPanel, div, Vec::new()),
                ],
            ),
            BlockId::ForTest1 => UIElement::new(Id::ForTest1, div, Vec::new()),
        }
    }
}
