use crate::app::{
    action_pump::Action, coords::*, input_state::InputState, observed::Observed, predefined::*,
    texture_manager::TextureManager, ui_element::*, ui_map::UIMap,
};

/// layers: root elements, z-indexed
pub struct UIManager {
    layers: Vec<Id32>,
    pub requires_update: bool,
    pub ui_scale: Observed<f32>,
}
impl UIManager {
    pub fn new(window_hight: u32) -> Self {
        Self {
            layers: vec![Id::RootMain.into()],
            requires_update: true,
            ui_scale: Observed::new(
                window_hight as f32 / 1080f32,
                Box::new(|s: f32| Action::UISizeObserve(s)),
            ),
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

            ui_map.elements.get_mut(root_id).transform = ui_map
                .aligns
                .get_mut(root_id)
                .apply(&mut win_size, self.ui_scale.get());

            //each root element is absolute, applied to a full window
            Self::update_rec(root_id, win_size, ui_map, self.ui_scale.get());
        }
        self.requires_update = false;
    }

    ///update self, then update children recursively
    fn update_rec(id: Id32, new_transform: XYWH, ui_map: &mut UIMap, ui_scale: f32) {
        ui_map.elements.get_mut(id).transform = new_transform;

        // windown that shrinks, after each ch.align (if ch is block)
        let mut dynamic_window = ui_map.elements.get(id).transform;
        for i in 0..ui_map.elements.get(id).childrens.len() {
            let ch_id = ui_map.elements.get(id).childrens[i];
            let ch_transform = ui_map
                .aligns
                .get(ch_id)
                .apply(&mut dynamic_window, ui_scale);
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
        for i in 0..self.layers.len() {
            ui_map
                .elements
                .get(self.layers[i])
                .draw_to(ui_map, t_manager);
        }

        t_manager.canvas.present();
    }
}
#[cfg(test)]
mod tests {
    use crate::ActionPump;

    use super::*;

    #[test]
    pub fn fit() {
        ActionPump::init();
        let win = XYWH::new(0, 0, 1000, 1000);
        let ui = &mut UIMap::new();
        UIManager::update_rec(Id::ForTest1.into(), win, ui, 1.0);
        assert_eq!(ui.elements.get(Id::ForTestSub1.into()).transform.x, 0);
        assert_eq!(ui.elements.get(Id::ForTestSub1.into()).transform.w, 400);
        assert_eq!(ui.elements.get(Id::ForTestSub2.into()).transform.x, 400);
        assert_eq!(ui.elements.get(Id::ForTestSub2.into()).transform.w, 600);
    }
}
