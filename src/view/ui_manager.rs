extern crate sdl2;

use super::ui_builder::{BlockId, UIBuilder};
use super::ui_element::{UIElement, UIElementTrait};
use super::{config::Config, coords::XYWH};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::RenderTarget;
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::WindowContext;

/// layers: z-indexed, root elements
pub struct UIManager {
    layers: Vec<Layer>,
    // ui_texture: Texture<'static>,
}
struct Layer {
    root_elem: UIElement,
}

/// responsible for building base of the UI
/// recieving draw calls from the loop
impl UIManager {
    pub fn new() -> Self {
        let main = UIBuilder::get(BlockId::MainLayout);
        Self {
            layers: vec![Layer { root_elem: main }],
            // ui_texture,
        }
    }

    pub fn update_pos(&mut self, window_size: XYWH, ctx: &Config) {
        for i in (0..self.layers.len()).rev() {
            &self.layers[i].root_elem.update_pos(window_size, ctx);
        }
    }

    pub fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        for i in 0..self.layers.len() {
            self.layers[i].root_elem.draw_to(canvas);
        }
    }
}
