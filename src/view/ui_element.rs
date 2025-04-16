use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    render::{Canvas, RenderTarget, Texture},
};

use super::{config::Config, coords::XYWH, div::Div, ui_builder::Id};

pub trait UIElementTrait {
    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>);
    fn update_pos(&mut self, transform: XYWH, ctx: &Config);
    fn get_id(&self) -> Id;
}

pub enum UIElement {
    Div(Div),
    None,
}
impl UIElementTrait for UIElement {
    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        match self {
            UIElement::Div(div) => div.draw_to(canvas),
            UIElement::None => (),
        }
    }
    fn update_pos(&mut self, transform: XYWH, ctx: &Config) {
        match self {
            UIElement::Div(div) => div.update_pos(transform, ctx),
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
