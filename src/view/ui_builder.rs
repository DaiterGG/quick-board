use sdl2::pixels::Color;

use super::{div::Div, ui_element::UIElement};

#[derive(Clone, Copy)]
/// unique name for each ui_element
pub enum Id {
    MainDiv,
    MainDivHeader,
    MainDivLeftPanel,
    MainDivRightPanel,

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
        match id {
            BlockId::MainLayout => UIElement::Div(Div::new(
                Id::MainDiv,
                None,
                vec![
                    UIElement::Div(Div::new(
                        Id::MainDivHeader,
                        Some(Color::RGB(255, 0, 0)),
                        Vec::new(),
                    )),
                    UIElement::Div(Div::new(
                        Id::MainDivLeftPanel,
                        Some(Color::RGB(0, 255, 0)),
                        Vec::new(),
                    )),
                    UIElement::Div(Div::new(
                        Id::MainDivRightPanel,
                        Some(Color::RGB(0, 0, 255)),
                        Vec::new(),
                    )),
                ],
            )),
            BlockId::ForTest1 => UIElement::Div(Div::new(Id::ForTest1, None, Vec::new())),
        }
    }
}
