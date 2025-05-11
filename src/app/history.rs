use std::cmp::min;

use sdl2::{rect::Rect, render::*, video::Window};

use crate::dl;

use super::{
    canvas_manager::CanvasData, coords::XYWH, history_step::HistoryStep, layer::Layer,
    texture_data::TextureData, texture_manager::TextureManager,
};

pub struct History {
    pub steps: Vec<HistoryStep>,
    pub selected_h_step: Option<usize>,
    pub layers: Vec<Layer>,
    pub selected_layer: Option<usize>,
}
impl History {
    pub fn new() -> Self {
        Self {
            selected_layer: None,
            layers: Vec::new(),
            steps: Vec::new(),
            selected_h_step: None,
        }
    }
    pub fn selected_step_mut(&mut self) -> &mut HistoryStep {
        if self.selected_h_step.is_none() {
            self.add_step();
        }
        &mut self.steps[self.selected_h_step.unwrap()]
    }
    pub fn add_step(&mut self) {
        let current_layer_id = self.selected_layer.unwrap_or(self.add_layer());
        dl!(self.layers.len());
        let current_layer = &mut self.layers[current_layer_id];

        if let Some(id) = self.selected_h_step {
            if id < self.steps.len() - 1 {
                self.steps.truncate(id + 1);
            }
        }
        let new_id = self.steps.len();

        if let Some(leaf_id) = current_layer.leaf_id {
            //modify leaf to point to new leaf
            self.steps[leaf_id].next_layer_step = Some(new_id);
            //add new step, that points to the old leaf
            self.steps.push(HistoryStep::new());
            //update layer
            current_layer.leaf_id = Some(new_id);
        } else {
            //add root and set it to the current layer
            self.steps.push(HistoryStep::new());

            current_layer.root_id = Some(new_id);
            current_layer.leaf_id = Some(new_id);
        }
        self.selected_h_step = Some(new_id);
    }
    pub fn add_layer(&mut self) -> usize {
        let insert_index = if let Some(s) = self.selected_layer {
            s + 1
        } else {
            self.selected_layer = Some(0);
            0
        };
        self.layers.insert(insert_index, Layer::new());
        self.selected_layer = Some(insert_index);
        insert_index
    }
    pub fn full_draw(
        &self,
        canvas: &mut Canvas<Window>,
        t_manager: &mut TextureManager,
        data: &CanvasData,
        dst: XYWH,
    ) {
        if self.selected_h_step.is_none() {
            return;
        }
        let src = XYWH::new(
            -min(0, (data.screen_pos.x as f32 / data.screen_zoom) as i32),
            -min(0, (data.screen_pos.y as f32 / data.screen_zoom) as i32),
            (dst.w as f32 / data.screen_zoom) as i32,
            (dst.h as f32 / data.screen_zoom) as i32,
        );
        for step_id in 0..(self.selected_h_step.unwrap() + 1) {
            if data.transform.w > 10_000 || data.transform.h > 10_000 {
                self.steps[step_id].full_draw_double(canvas, t_manager, data, src, dst);
            } else {
                self.steps[step_id].full_draw(canvas, t_manager, data, src, dst);
            }
        }
    }
    pub fn finish_step(&mut self, t_manager: &mut TextureManager, canvas: &mut Canvas<Window>) {
        self.steps[self.selected_h_step.unwrap()].make_static(t_manager, canvas);
    }
}
