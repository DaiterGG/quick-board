use std::env::JoinPathsError;

use crate::{app::action_pump::Action, dl};

use super::{
    action_pump::ActionPump, coords::XY, input_state::*, predefined::*, style_align::Align,
    ui_map::UIMap,
};

pub struct Slider {
    pub handle_id: IdI32,
    pub handle_within_slider: bool,
}
impl Slider {
    pub fn new(handle_id: IdI32) -> Self {
        Self {
            handle_id,
            handle_within_slider: false,
        }
    }
    pub fn within(mut self) -> Self {
        self.handle_within_slider = true;
        self
    }
    pub fn before_collision(id: IdI32, input: &mut InputState, ui: &mut UIMap, hit: bool) {
        let this_data = &ui.sliders_data[&id];
        let this_tr = ui.elements[id as usize].transform;
        if hit && input.left() == ButtonState::Pressed {
            input.interacting_with = Some(id);
        }
        if input.interacting_with == Some(id) {
            let (x, w) = (this_tr.x as f32, this_tr.w as f32);
            let ratio = (input.pos.x as f32 - x) / w;
            let percent = (ratio * 100.0) as i32;

            let handle = &mut ui.elements[this_data.handle_id as usize];
            let handle_a = &mut ui.aligns[this_data.handle_id as usize];

            if let Align::Absolute {
                align_by, pivot, ..
            } = handle_a
            {
                align_by.x = percent.clamp(0, 100);
                let half = handle.transform.w as f32 / 2.0;
                let handle_pos = input.pos.x as f32 - half;

                let pos_clamped = if this_data.handle_within_slider {
                    pivot.x = percent.clamp(0, 100);

                    handle_pos.clamp(x, x + w - handle.transform.w as f32)
                } else {
                    handle_pos.clamp(x - half, x + w - half)
                };
                handle.transform.x = pos_clamped as i32;
            } else {
                panic!("type block is not supported for handle of slider");
            }
            ActionPump::add(Action::SliderLine(id, ratio.clamp(0.0, 1.0)));
        }
    }
    // if input.interacting_with == Some(id) {
    //     let mut padding_ratio = 0.0;
    //     let handle = &mut ui.elements[this_data.handle_id as usize];
    //     let (x, w) = match this_data.handle_within_slider {
    //         true => {
    //             padding_ratio = handle.transform.w as f32 / this_tr.w as f32;
    //             (
    //                 this_tr.x as f32 + handle.transform.w as f32 / 2.0,
    //                 this_tr.w as f32 - handle.transform.w as f32,
    //             )
    //         }
    //         false => (this_tr.x as f32, this_tr.w as f32),
    //     };
    //     let ptr_x = input.pos.x as f32;
    //     let ratio = (ptr_x - x) / w;
    //     let padding_percent = (ratio - 0.5) * padding_ratio;
    //     let percent = ratio * 100.0 - padding_percent;

    //     let handle_a = &mut ui.aligns[this_data.handle_id as usize];
    //     if let Align::Absolute { align_by, .. } = handle_a {
    //         align_by.x = (percent as i32).clamp(0, 100);
    //     } else {
    //         panic!("type block is not supported for handle of slider");
    //     }
    //     let pos_clamped = ptr_x.clamp(x, x + w);
    //     handle.transform.x = (pos_clamped - handle.transform.w as f32 / 2.0) as i32;
    //     // actions.add(Action::Drag(id, delta));
    // }
}
