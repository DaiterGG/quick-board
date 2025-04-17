use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    render::{Canvas, RenderTarget, Texture},
};

use super::{
    app::App, coords::XYWH, div::Div, states::States, style_map::StyleMap, ui_builder::Id,
};

pub trait UIElementTrait {
    fn update_pos(&mut self, transform: XYWH, styles: &StyleMap, states: &mut States);
    fn pointer_collision(&self, states: &mut States);
    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>);
    fn get_id(&self) -> Id;
}

pub enum UIElement {
    Div(Div),
    None,
}
impl UIElementTrait for UIElement {
    fn update_pos(&mut self, transform: XYWH, styles: &StyleMap, states: &mut States) {
        match self {
            UIElement::Div(div) => div.update_pos(transform, &styles, states),
            UIElement::None => (),
        }
    }
    fn pointer_collision(&self, states: &mut States) {
        match self {
            UIElement::Div(div) => div.pointer_collision(states),
            UIElement::None => (),
        }
    }
    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        match self {
            UIElement::Div(div) => div.draw_to(canvas),
            UIElement::None => (),
        }
    }
    fn get_id(&self) -> Id {
        match self {
            UIElement::Div(div) => div.id,
            UIElement::None => Id::MainDiv,
        }
    }
}

impl UIElement {
    /// unsafe, for testing
    pub fn unwrap_div(&self) -> &Div {
        match self {
            UIElement::Div(div) => div,
            _ => {
                panic!("failed to unwrap div");
            }
        }
    }
}
