use super::{
    coords::XYWH,
    states::States,
    style_map::StyleMap,
    ui_builder::{BlockId, UIBuilder},
    ui_element::{UIElement, UIElementTrait},
};

use sdl2::render::{Canvas, RenderTarget};

/// layers: z-indexed, root elements
pub struct UIManager {
    layers: Vec<UIElement>,
    styles: StyleMap,
}
/// responsible for building base of the UI
/// recieving draw calls from the loop
impl UIManager {
    pub fn new() -> Self {
        let main = UIBuilder::get(BlockId::MainLayout);
        Self {
            layers: vec![main],
            styles: StyleMap::new_first(),
        }
    }

    pub fn update(&mut self, states: &mut States) {
        if states.ui.requires_update {
            let window_size = XYWH::new(0, 0, states.ui.window_size.w, states.ui.window_size.h);
            for i in (0..self.layers.len()).rev() {
                self.layers[i].update_pos(window_size, &self.styles, states);
            }
            states.ui.requires_update = false;
        }
    }

    pub fn pointer_collision(&self, states: &mut States) {
        if states.pointer.updated {
            for i in 0..self.layers.len() {
                self.layers[i].pointer_collision(states);
            }
        }
    }
    pub fn draw_ui<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        for i in 0..self.layers.len() {
            self.layers[i].draw_to(canvas);
        }
    }
}
