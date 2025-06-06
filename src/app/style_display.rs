use sdl2::rect::Rect;

use crate::dl;

use super::border::Border;
use super::color_display::ColorDisplay;
use super::color_map::*;
use super::coords::*;
use super::texture_manager::*;
use super::texture_vec::TexId;
use super::texture_vec::TexId16;

#[derive(Debug, Copy, Clone)]
pub enum DisplayState {
    Idle,
    // Active,
    // Disabled,
    Hovered,
    Pressed,
    Held,
    Released,
    Total,
}
#[derive(Copy, Clone)]
/// active_states: current element state
/// states_data: constant settings
pub struct Display {
    pub active_states: [bool; DisplayState::Total as usize],
    pub states_data: [Option<DisplayData>; DisplayState::Total as usize],
}

impl Display {
    /// for building Display without idle state
    /// for empty Display, use Option<Display> = None
    pub fn none() -> Self {
        let mut dis = Display {
            active_states: [false; DisplayState::Total as usize],
            states_data: [const { None }; DisplayState::Total as usize],
        };
        dis.active_states[DisplayState::Idle as usize] = true;
        dis
    }
    pub fn idle(idle: DisplayData) -> Self {
        let mut dis = Display {
            active_states: [false; DisplayState::Total as usize],
            states_data: [const { None }; DisplayState::Total as usize],
        };
        dis.states_data[DisplayState::Idle as usize] = Some(idle);
        dis.active_states[DisplayState::Idle as usize] = true;
        dis
    }

    pub fn hovered(mut self, data: DisplayData) -> Self {
        self.states_data[DisplayState::Hovered as usize] = Some(data);
        self
    }
    pub fn pressed(mut self, data: DisplayData) -> Self {
        self.states_data[DisplayState::Pressed as usize] = Some(data);
        self
    }
    pub fn released(mut self, data: DisplayData) -> Self {
        self.states_data[DisplayState::Released as usize] = Some(data);
        self
    }
    pub fn held(mut self, data: DisplayData) -> Self {
        self.states_data[DisplayState::Held as usize] = Some(data);
        self
    }
    pub fn set_state(&mut self, state: DisplayState, active: bool) {
        self.active_states[state as usize] = active;
    }
    pub fn draw(
        &self,
        pos: XYWH,
        at_front: bool,
        colors: &ColorMap,
        textures: &mut TextureManager,
    ) {
        for i in 0..DisplayState::Total as usize {
            if self.active_states[i] {
                if let Some(data) = &self.states_data[i] {
                    if data.draw_at_front != at_front {
                        continue;
                    }
                    data.draw(pos, colors, textures);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DisplayData {
    draw_at_front: bool,
    color: Option<ColorDisplay>,
    sub_color: Option<ColorDisplay>,
    pub tex_id: Option<TexId16>,
    edge_radius: u8,
    border: Option<Border>,
}
impl DisplayData {
    pub fn transparent() -> Self {
        DisplayData {
            draw_at_front: false,
            tex_id: None,
            color: None,
            sub_color: None,
            edge_radius: 0,
            border: None,
        }
    }
    /// to draw behind childrens
    pub fn new(color: ColorTag) -> Self {
        DisplayData {
            draw_at_front: false,
            tex_id: None,
            color: Some(ColorDisplay::full(color)),
            sub_color: None,
            edge_radius: 0,
            border: None,
        }
    }
    /// to draw on top of the childrens
    pub fn at_front(mut self) -> Self {
        self.draw_at_front = true;
        self
    }
    pub fn sub(mut self, sub: ColorTag, alfa: u8) -> Self {
        self.sub_color = Some(ColorDisplay::with_alfa(sub, alfa));
        self
    }
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
    pub fn with_tex(mut self, id: TexId) -> Self {
        self.tex_id = Some(id.into());
        self
    }
    pub fn set_tex(&mut self, id: TexId16) {
        self.tex_id = Some(id);
    }
    pub fn radius(mut self, radius: u8) -> Self {
        self.edge_radius = radius;
        self
    }
    fn draw(&self, element_pos: XYWH, colors: &ColorMap, t_manager: &mut TextureManager) {
        if let Some(main) = self.color {
            main.apply(&mut t_manager.canvas, colors, element_pos);
        }

        if let Some(id) = self.tex_id {
            Self::apply_texture(t_manager, id, element_pos);
        }

        if let Some(sub) = self.sub_color {
            sub.apply(&mut t_manager.canvas, colors, element_pos);
        }

        if let Some(bord) = self.border {
            bord.apply(&mut t_manager.canvas, colors, element_pos);
        }
    }
    fn apply_texture(t_manager: &mut TextureManager, id: TexId16, element_pos: XYWH) {
        let data = t_manager.textures.get(id);
        let dst = if let Some(dst) = data.dst {
            Some(Rect::new(
                element_pos.x + dst.x,
                element_pos.y + dst.y,
                dst.w as u32,
                dst.h as u32,
            ))
        } else {
            Some(element_pos.to_rect())
        };
        let src = data.src.map(|src| src.to_rect());
        t_manager.canvas.copy(&data.texture, src, dst).unwrap();
    }
}
