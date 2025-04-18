use super::{
    action_state::Action,
    coords::XYWH,
    states::States,
    style_map::StyleMap,
    ui_builder::{BlockId, Id, UIBuilder},
    ui_element::UIElement,
};

use sdl2::{
    pixels::Color,
    render::{Canvas, RenderTarget},
};

/// layers: root elements, z-indexed
pub struct UIManager {
    layers: Vec<UIElement>,
    styles: StyleMap,
}
/// responsible for building base of the UI
impl UIManager {
    pub fn new() -> Self {
        let main = UIBuilder::get(BlockId::MainLayout);
        Self {
            layers: vec![main],
            styles: StyleMap::new(),
        }
    }

    /// called once per frame
    pub fn update(&mut self, states: &mut States) {
        states.action.add(Action::ButtonPressed(Id::MainDiv));
        if states.ui.requires_update() {
            let window_size = XYWH::new(0, 0, states.ui.window_size.w, states.ui.window_size.h);
            let styles = &mut self.styles;
            for i in 0..self.layers.len() {
                let mut full = window_size.clone();
                let layer = &mut self.layers[i];

                //each root element is absolute, applied to a full window
                layer.transform = styles.get_align(layer.id).align(&mut full, states);
                layer.update_childrens(styles, states);
            }
        }
    }

    /// called once per frame
    pub fn pointer_collision(&mut self, states: &mut States) {
        // NOTE: This if requires a bunch of maitenence,
        // even for just one frame animations (Pressed -> Held)
        // if states.pointer.updated ||
        //     // edge case, when ui updated under the pointer, but pointer was not moving
        //     states.ui.was_updated_last_frame()
        // {
        for i in 0..self.layers.len() {
            self.layers[i].pointer_collision(states, &mut self.styles, true);
        }
        // }
    }

    /// called once per frame
    pub fn draw_ui<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // let dis = self.styles.get_display(self.layers[0].id);
        // dis.inspect(|d| println!("{:?}", d.active_states));

        for i in 0..self.layers.len() {
            self.layers[i].draw_to(canvas, &self.styles);
        }

        canvas.present();
    }
}
