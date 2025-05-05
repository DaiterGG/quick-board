use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

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
        let id = self.selected_h_step.unwrap_or_else(|| self.add_step());
        &mut self.steps[id]
    }
    pub fn add_step(&mut self) -> usize {
        let current_layer_id = self.selected_layer.unwrap_or(self.add_layer());
        let current_layer = &mut self.layers[current_layer_id];

        // todo: delete if not the last step selected
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
        new_id
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
        canvas_data: &CanvasData,
        src: XYWH,
        dst: XYWH,
    ) {
        for step in &self.steps {
            step.full_draw(canvas, t_manager, canvas_data, src, dst);
        }
    }
}
