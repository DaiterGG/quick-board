use std::cmp::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode::Blend;
use sdl2::render::*;
use sdl2::surface;
use sdl2::video::Window;

use super::color_map::*;
use super::coords::*;
use super::texture_manager::*;

#[derive(Debug)]
pub enum DisplayState {
    Idle,
    Active,
    Disabled,
    Hovered,
    Pressed,
    Held,
    Released,
    Total,
}
#[derive(Clone)]
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
    pub fn hovered(&mut self, data: DisplayData) -> &mut Self {
        self.states_data[DisplayState::Hovered as usize] = Some(data);
        self
    }
    pub fn pressed(&mut self, data: DisplayData) -> &mut Self {
        self.states_data[DisplayState::Pressed as usize] = Some(data);
        self
    }
    pub fn released(&mut self, data: DisplayData) -> &mut Self {
        self.states_data[DisplayState::Released as usize] = Some(data);
        self
    }
    pub fn held(&mut self, data: DisplayData) -> &mut Self {
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
        canvas: &mut Canvas<Window>,
        colors: &ColorMap,
        textures: &TextureManager,
    ) {
        // println!("{:?}", self.active_states);
        // println!("{:?}", self.states_data);
        for i in 0..DisplayState::Total as usize {
            if self.active_states[i] {
                if let Some(data) = &self.states_data[i] {
                    if data.draw_at_front != at_front {
                        continue;
                    }
                    data.draw(pos, canvas, colors, textures);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct DisplayData {
    draw_at_front: bool,
    color: Option<ColorDisplay>,
    sub_color: Option<ColorDisplay>,
    texture_id: TexId,
    edge_radius: u8,
    border: Option<Border>,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TexId {
    Locked(LockedTexId),
    Open(i32),
    None,
}
// impl Clone for DisplayData {
//     fn clone(&self) -> Self {
//         assert!(self.texture_id == TexId::None);
//         DisplayData {
//             texture_id: TexId::None,
//             draw_at_front: self.draw_at_front,
//             color: self.color,
//             sub_color: self.sub_color,
//             edge_radius: self.edge_radius,
//             border: self.border.clone(),
//         }
//     }
// }
impl DisplayData {
    pub fn transparent() -> Self {
        DisplayData {
            draw_at_front: false,
            texture_id: TexId::None,
            color: None,
            sub_color: None,
            edge_radius: 0,
            border: None,
        }
    }
    /// for draw behind childrens
    pub fn bg(color: ColorTag) -> Self {
        DisplayData {
            draw_at_front: false,
            texture_id: TexId::None,
            color: Some(ColorDisplay::full(color)),
            sub_color: None,
            edge_radius: 0,
            border: None,
        }
    }
    /// for draw on top of the childrens
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
    pub fn locked_texture(mut self, id: LockedTexId) -> Self {
        self.texture_id = TexId::Locked(id);
        self
    }
    pub fn open_texture(mut self, id: usize) -> Self {
        self.texture_id = TexId::Open(id as i32);
        self
    }
    pub fn radius(mut self, radius: u8) -> Self {
        self.edge_radius = radius;
        self
    }
    fn draw(
        &self,
        element_pos: XYWH,
        canvas: &mut Canvas<Window>,
        colors: &ColorMap,
        textures: &TextureManager,
    ) {
        if let Some(main) = self.color {
            main.apply(canvas, colors, element_pos);
        }

        match self.texture_id {
            // TexId::Locked(id) => {
            //     let data = textures.locked_texture(id);
            //     let dst = if let Some(dst) = data.dst {
            //         Some(dst)
            //     } else {
            //         Some(element_pos.to_rect())
            //     };
            //     let _ = canvas.copy(&data.texture, data.src, dst);
            // }
            TexId::Open(id) => {
                let data = &textures.open_textures[id as usize];
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
                let src = if let Some(src) = data.src {
                    Some(src.to_rect())
                } else {
                    None
                };
                let _ = canvas.copy(&data.texture, src, dst);
            }
            _ => {}
        }

        if let Some(sub) = self.sub_color {
            sub.apply(canvas, colors, element_pos);
        }
        // TODO: Draw border
        // canvas.rect(
        //     XYWH::new(pos.x, pos.y, pos.w, pos.h),
        //     border_color,
        //     border_width,
        // );
    }
}
#[derive(Copy, Clone, Debug)]
struct Border {
    color: ColorTag,
    width: u8,
}
#[derive(Copy, Clone)]
struct ColorDisplay {
    color: ColorTag,
    alfa: u8,
}
impl ColorDisplay {
    pub fn full(color: ColorTag) -> Self {
        Self { color, alfa: 255 }
    }
    pub fn with_alfa(color: ColorTag, alfa: u8) -> Self {
        Self { color, alfa }
    }
    pub fn apply<T: RenderTarget>(&self, canvas: &mut Canvas<T>, colors: &ColorMap, pos: XYWH) {
        let mut rgba: Color = colors.get(self.color);
        rgba.a = self.alfa as u8;
        canvas.set_draw_color(rgba);
        canvas.set_blend_mode(Blend);
        let _ = canvas.fill_rect(Rect::new(pos.x, pos.y, pos.w as u32, pos.h as u32));
    }
}
