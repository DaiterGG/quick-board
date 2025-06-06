use std::cmp::min;

use sdl2::{
    pixels::Color,
    render::{BlendMode, TextureAccess},
};

use crate::{d, dl};

use super::{
    canvas_manager::CanvasData,
    coords::{WH, XYWH},
    history_step::HistoryStep,
    layer::Layer,
    texture_data::TextureData,
    texture_manager::TextureManager,
    texture_vec::TexId16,
};

pub struct History {
    pub steps: Vec<HistoryStep>,
    pub selected_h_step: Option<usize>,
    pub layers: Vec<Layer>,
    pub selected_layer: Option<usize>,

    // TODO: remove this when layers are done
    temp_tex: Option<TexId16>,
}
impl History {
    pub fn new() -> Self {
        Self {
            selected_layer: None,
            layers: Vec::new(),
            steps: Vec::new(),
            selected_h_step: None,
            temp_tex: None,
        }
    }
    /// returns None if there is no selected step
    pub fn try_get_target_step(&mut self) -> Option<&mut HistoryStep> {
        let step = &mut self.steps[self.selected_h_step?];
        if step.is_static { None } else { Some(step) }
    }
    pub fn add_step(&mut self, is_eraser: bool) -> &mut HistoryStep {
        let current_layer_id = self.selected_layer.unwrap_or(self.add_layer());
        let current_layer = &mut self.layers[current_layer_id];

        let id = if let Some(id) = self.selected_h_step {
            id as i32
        } else {
            -1
        };
        if id < (self.steps.len() as i32) - 1 {
            self.steps.truncate((id + 1) as usize);
        }
        let new_id = self.steps.len();

        self.selected_h_step = Some(new_id);

        self.set_leaf_to_current();
        self.steps.push(HistoryStep::new(is_eraser));
        &mut self.steps[new_id]
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

    pub fn undo(&mut self, t_manager: &mut TextureManager) {
        if let Some(id) = self.selected_h_step {
            self.finish_step(t_manager);
            if id > 0 {
                self.selected_h_step = Some(id - 1);
            } else {
                self.selected_h_step = None;
            }
            self.set_leaf_to_current();
        }
    }
    pub fn redo(&mut self) {
        if let Some(id) = self.selected_h_step {
            if id < self.steps.len() - 1 {
                self.selected_h_step = Some(id + 1);
            }
        } else {
            self.selected_h_step = Some(0);
        }
        self.set_leaf_to_current();
    }
    /// maintain current layer root and leaf
    pub fn set_leaf_to_current(&mut self) {
        if self.selected_h_step.is_none() {
            return;
        }
        let layer_id = self.selected_layer.unwrap();

        let layer = &mut self.layers[layer_id];
        if let Some(h_step_id) = self.selected_h_step {
            if let Some(leaf_id) = layer.leaf_id {
                self.steps[leaf_id].next_layer_step = Some(h_step_id);
                layer.leaf_id = Some(h_step_id);
                self.steps[h_step_id].next_layer_step = None;
            } else {
                layer.root_id = Some(h_step_id);
                layer.leaf_id = Some(h_step_id);
            }
            return;
        }

        if let Some(root_id) = layer.root_id {
            self.steps[root_id].next_layer_step = None;
        }
        if let Some(leaf_id) = layer.leaf_id {
            self.steps[leaf_id].next_layer_step = None;
        }
        layer.root_id = None;
        layer.leaf_id = None;
    }
    pub fn full_draw(data: &mut CanvasData, t_manager: &mut TextureManager, dst: XYWH) {
        let this = &mut data.history;

        // TODO: remove this when layers are done
        let temp_tex = if let Some(temp_tex) = this.temp_tex {
            temp_tex
        } else {
            let mut td = TextureData::new(
                &t_manager.t_creator,
                WH::new_one(t_manager.biggest_possible_resolution),
                None,
                Some(TextureAccess::Target),
            );
            td.texture.set_blend_mode(BlendMode::Blend);
            let temp_tex = t_manager.textures.init_texture(td);
            this.temp_tex = Some(temp_tex);
            temp_tex
        };
        t_manager
            .canvas
            .with_texture_canvas(&mut t_manager.textures.get_mut(temp_tex).texture, |c| {
                c.set_draw_color(Color::RGBA(0, 0, 0, 0));
                c.clear();
            })
            .unwrap();

        if this.selected_h_step.is_none() {
            // TODO: remove this when layers are done
            t_manager
                .canvas
                .with_texture_canvas(
                    &mut t_manager.textures.get_mut(data.targeted_ui_texture).texture,
                    |c| {
                        c.set_draw_color(Color::RGBA(20, 20, 20, 255));
                        c.clear();
                    },
                )
                .unwrap();
            return;
        }

        // TODO: remove this when layers are done
        let orig_target = data.targeted_ui_texture;
        data.targeted_ui_texture = temp_tex;

        let src = XYWH::new(
            -min(0, (data.screen_pos.x as f32 / data.screen_zoom) as i32),
            -min(0, (data.screen_pos.y as f32 / data.screen_zoom) as i32),
            (dst.w as f32 / data.screen_zoom) as i32,
            (dst.h as f32 / data.screen_zoom) as i32,
        );

        for step_id in 0..(this.selected_h_step.unwrap() + 1) {
            // if data.transform.w > 10_000 || data.transform.h > 10_000 {
            //     self.steps[step_id].full_draw_double(t_manager, data, src, dst);
            // } else {
            data.history.steps[step_id].full_draw(t_manager, data, src, dst);
            // }
        }

        // TODO: remove this when layers are done
        let (tex, temp) = t_manager
            .textures
            .get_mut_2(orig_target, data.history.temp_tex.unwrap());
        t_manager
            .canvas
            .with_texture_canvas(&mut tex.texture, |c| {
                c.set_draw_color(Color::RGBA(20, 20, 20, 255));
                c.clear();
                c.copy_f(&temp.texture, None, None).unwrap();
            })
            .unwrap();

        data.targeted_ui_texture = orig_target;
    }
    pub fn finish_step(&mut self, t_manager: &mut TextureManager) {
        if self.selected_h_step.is_none() {
            return;
        }
        self.steps[self.selected_h_step.unwrap()].make_static(t_manager);
    }
}
