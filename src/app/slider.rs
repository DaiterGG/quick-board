use std::env::JoinPathsError;

use crate::{app::action_pump::Action, dl};

use super::{
    action_pump::ActionPump, coords::XY, input_state::*, predefined::*, style_align::Align,
    ui_map::UIMap,
};

pub struct Slider {
    pub handle_id: Id32,
    pub handle_within_slider: bool,
}
impl Slider {
    pub fn new(handle_id: Id32) -> Self {
        Self {
            handle_id,
            handle_within_slider: false,
        }
    }
    pub fn within(mut self) -> Self {
        self.handle_within_slider = true;
        self
    }
    /// called when callback, this slider subscribed to, is triggered
    pub fn update(new_val: f32, id: Id32, ui: &mut UIMap) {
        let this = &ui.elements_data[&id]
            .downcast_ref::<Slider>()
            .unwrap_or_else(|| {
                panic!(
                    "element - '{:?}' is subscribed to a callback that expects a slider",
                    id
                )
            });

        let percent = ((new_val * 100.0) as i32).clamp(0, 100);

        this.apply_align(percent, ui.aligns.get_mut(this.handle_id));
    }
    pub fn before_collision(id: Id32, input: &mut InputState, ui: &mut UIMap, hit: bool) {
        if hit && input.left() == ButtonState::Pressed {
            input.interacting_with = Some(id);
        }
        if input.interacting_with == Some(id) {
            let this = ui.elements_data[&id]
                .downcast_ref::<Slider>()
                .expect("non slider data was put in slider");
            let this_tr = ui.elements.get(id).transform;

            let padding = if this.handle_within_slider {
                ui.elements.get(this.handle_id).transform.w as f32
            } else {
                0.0
            };

            let (x, w) = (this_tr.x as f32, this_tr.w as f32);
            let ratio = (input.pos.x as f32 - (x + padding / 2.0)) / (w - padding);
            let percent = ((ratio * 100.0) as i32).clamp(0, 100);

            this.apply_align(percent, &mut ui.aligns.get_mut(this.handle_id));

            ActionPump::add(Action::SliderLine(id, ratio.clamp(0.0, 1.0)));
        }
    }
    fn apply_align(&self, percent: i32, align: &mut Align) {
        if let Align::Absolute {
            align_by, pivot, ..
        } = align
        {
            align_by.x = percent;
            if self.handle_within_slider {
                pivot.x = percent;
            };
        } else {
            panic!("type block is not supported for handle of slider");
        }
        // NOTE: can be updated directly in the future
        ActionPump::add(Action::UIUpdate);
    }
    fn update_handle(id: Id32, ui: &mut UIMap) {
        let this = &ui.elements_data[&id]
            .downcast_ref::<Slider>()
            .unwrap_or_else(|| {
                panic!(
                    "element - '{:?}' is subscribed to a callback that expects a slider",
                    id
                )
            });

        let handle_d = ui.displays.get_mut(this.handle_id);
    }
}
