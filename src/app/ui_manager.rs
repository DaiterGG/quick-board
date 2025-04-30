use crate::app::{pointer_state::ButtonState, style_display::DisplayState};

use super::{
    action_pump::ActionPump, button::Button, coords::*, draw_window::DrawWindow,
    element_map::ElementMap, pointer_state::PointerState, predefined::*,
    texture_manager::TextureManager, ui_element::*, ui_map::UIMap,
};

use sdl2::{
    pixels::Color,
    render::{Canvas, RenderTarget},
};

/// layers: root elements (ids), z-indexed
pub struct UIManager {
    pub window_size: WH,
    layers: Vec<IdUsize>,
    pub requires_update: bool,
    ui_scale: f32,
}
impl UIManager {
    pub fn new(window_size: WH) -> Self {
        Self {
            layers: ElementMap::init_layers(),
            requires_update: true,
            ui_scale: window_size.h as f32 / 1080f32,
            window_size,
        }
    }
    /// called once per frame
    pub fn update(&mut self, ui_map: &mut UIMap) {
        // states.action.add(Action::ButtonPressed(Id::MainDiv));
        if self.requires_update {
            let full_window = XYWH::new(0, 0, self.window_size.w, self.window_size.h);
            for i in 0..self.layers.len() {
                let root_id = self.layers[i];

                let mut win_size = full_window.clone();

                ui_map.element_mut(root_id).transform =
                    ui_map.align(root_id).apply(&mut win_size, self.ui_scale);

                //each root element is absolute, applied to a full window
                Self::update_rec(root_id, win_size, ui_map, self.ui_scale);
                // UIElement::update_as_root(root_id, full_window, self.get_current_ui_scale());
            }
            self.requires_update = false;
        }
    }

    ///update self, then update children recursively
    fn update_rec(id: IdUsize, new_transform: XYWH, ui_map: &mut UIMap, ui_scale: f32) {
        ui_map.element_mut(id).transform = new_transform;

        // windown that shrinks, after each ch.align (if ch is block)
        let mut dynamic_window = ui_map.element(id).transform.clone();
        for i in 0..ui_map.element(id).childrens.len() {
            let ch: usize = ui_map.element(id).childrens[i];
            let ch_transform = ui_map.align(ch).apply(&mut dynamic_window, ui_scale);
            Self::update_rec(ch, ch_transform, ui_map, ui_scale);
        }
    }

    /// called once per frame
    pub fn pointer_collision(
        &mut self,
        pointer: &mut PointerState,
        actions: &mut ActionPump,
        ui_map: &mut UIMap,
    ) {
        // NOTE: This if requires a bunch of maitenence,
        // even for just one frame animations (button Pressed -> Held)
        // if states.pointer.updated ||
        //     // edge case, when ui updated under the pointer, but pointer was not moving
        //     states.ui.was_updated_last_frame()
        // {
        let mut prev_hit = false;
        // iterate z-index wise
        for i in (0..self.layers.len()).rev() {
            // if front layer was hit, rest of the layers can't be hit
            prev_hit =
                Self::pointer_collision_rec(self.layers[i], ui_map, pointer, !prev_hit, actions);
        }
        // }
    }

    fn pointer_collision_rec(
        id: IdUsize,
        ui_map: &mut UIMap,
        pointer: &mut PointerState,
        parrent_hit: bool,
        actions: &mut ActionPump,
    ) -> bool {
        // let element = ui_map.element(id);
        //if parrent wasn't hit, then children are not calculated
        let hit = parrent_hit && ui_map.element(id).transform.is_within(pointer.pos);

        if let Some(dis) = ui_map.display_mut(id) {
            dis.set_state(DisplayState::Hovered, hit);
            dis.set_state(
                DisplayState::Pressed,
                pointer.left == ButtonState::Pressed && hit,
            );
            dis.set_state(DisplayState::Held, pointer.left == ButtonState::Held && hit);
            dis.set_state(
                DisplayState::Released,
                pointer.left == ButtonState::Released && hit,
            );
            // println!("{:?}", dis.active_states);
            // println!("{:?}", states.pointer.left);
        }
        // element specific logic
        match ui_map.element(id).element_type {
            ElementType::Button if hit => {
                Button::before_collision(id, actions, pointer);
            }
            ElementType::DrawWindow => {
                DrawWindow::before_collision(id, ui_map.element(id), actions, pointer, hit);
            }
            _ => {} //div
        }

        for i in 0..ui_map.element(id).childrens.len() {
            Self::pointer_collision_rec(
                ui_map.element(id).childrens[i],
                ui_map,
                pointer,
                hit,
                actions,
            );
        }

        // element specific logic
        match ui_map.element(id).element_type {
            ElementType::Button if hit => {
                // Button::after_collision(ui.element(id), states);
            }
            _ => {} //div
        }
        // for ui_manager check
        hit
    }
    /// called once per frame
    pub fn draw_ui<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        ui_map: &UIMap,
        textures: &TextureManager,
    ) {
        canvas.set_draw_color(Color::RGB(14, 14, 14));
        canvas.clear();

        // let dis = self.styles.get_display(self.layers[0].id);
        // dis.inspect(|d| println!("{:?}", d.active_states));

        // TODO: draw draw_canvas here
        //
        for i in 0..self.layers.len() {
            ui_map
                .element(self.layers[i])
                .draw_to(canvas, ui_map, textures);
        }

        canvas.present();
    }
}
