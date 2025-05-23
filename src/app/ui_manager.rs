use crate::dl;

use super::{
    action_pump::ActionPump, coords::*, element_map::ElementMap, input_state::InputState,
    predefined::*, texture_manager::TextureManager, ui_element::*, ui_map::UIMap,
};

use sdl2::pixels::Color;

/// layers: root elements (ids), z-indexed
pub struct UIManager {
    layers: Vec<IdI32>,
    pub requires_update: bool,
    pub ui_scale: f32,
}
impl UIManager {
    pub fn new(window_hight: u32) -> Self {
        Self {
            layers: ElementMap::init_layers(),
            requires_update: true,
            ui_scale: window_hight as f32 / 1080f32,
        }
    }
    /// called once per frame
    pub fn update(&mut self, ui_map: &mut UIMap, t_manager: &mut TextureManager) {
        if !self.requires_update {
            return;
        }
        let w_size = t_manager.canvas.window().size();
        let full_window = XYWH::new(0, 0, w_size.0 as i32, w_size.1 as i32);
        for i in 0..self.layers.len() {
            let root_id = self.layers[i];

            let mut win_size = full_window;

            ui_map.elements[root_id as usize].transform =
                ui_map.aligns[root_id as usize].apply(&mut win_size, self.ui_scale);

            //each root element is absolute, applied to a full window
            Self::update_rec(root_id, win_size, ui_map, self.ui_scale);
        }
        self.requires_update = false;
    }

    ///update self, then update children recursively
    fn update_rec(id: IdI32, new_transform: XYWH, ui_map: &mut UIMap, ui_scale: f32) {
        ui_map.elements[id as usize].transform = new_transform;

        // windown that shrinks, after each ch.align (if ch is block)
        let mut dynamic_window = ui_map.elements[id as usize].transform;
        for i in 0..ui_map.elements[id as usize].childrens.len() {
            let ch_id = ui_map.elements[id as usize].childrens[i];
            let ch_transform = ui_map.aligns[ch_id as usize].apply(&mut dynamic_window, ui_scale);
            Self::update_rec(ch_id, ch_transform, ui_map, ui_scale);
        }
    }

    /// called once per frame
    pub fn pointer_collision(&mut self, input: &mut InputState, ui_map: &mut UIMap) {
        let mut prev_hit = false;
        // iterate z-index wise
        for i in (0..self.layers.len()).rev() {
            // if front layer was hit, rest of the layers can't be hit
            prev_hit = UIElement::pointer_collision_rec(self.layers[i], ui_map, input, !prev_hit);
        }
    }

    /// called once per frame
    pub fn draw_ui(&self, ui_map: &UIMap, t_manager: &mut TextureManager) {
        t_manager.canvas.set_draw_color(Color::RGB(14, 14, 14));
        // canvas.set_draw_color(Color::RGB(14, 14, 14));
        t_manager.canvas.clear();

        // let dis = self.styles.get_display(self.layers[0].id);
        // dis.inspect(|d| println!("{:?}", d.active_states));

        for i in 0..self.layers.len() {
            ui_map.elements[self.layers[i] as usize].draw_to(ui_map, t_manager);
        }

        t_manager.canvas.present();
    }
}
