use std::i32;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use super::color_map::{ColorMap, ColorTag};
use super::coords::{WH, XY, XYWH};
use super::states::States;
use super::style_map::StyleMap;
use super::ui_element::UIElement;

const STATES_COUNT: usize = 7;
#[derive(Debug)]
pub enum DisplayState {
    Idle,
    Active,
    Disabled,
    Hovered,
    Pressed,
    Held,
    Released,
}
#[derive(Copy, Clone, Debug)]
/// active_states: current element state
/// states_data: constant settings
pub struct Display {
    pub active_states: [bool; STATES_COUNT],
    states_data: [Option<DisplayData>; STATES_COUNT],
}

impl Display {
    /// for building Display without idle state
    /// for empty Display, use Option<Display> = None
    pub fn none() -> Self {
        let mut dis = Display {
            active_states: [false; STATES_COUNT],
            states_data: [None; STATES_COUNT],
        };
        dis.active_states[DisplayState::Idle as usize] = true;
        dis
    }
    pub fn new(idle: DisplayData) -> Self {
        let mut dis = Display {
            active_states: [false; STATES_COUNT],
            states_data: [None; STATES_COUNT],
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
    pub fn draw<T: RenderTarget>(
        &self,
        pos: XYWH,
        at_front: bool,
        canvas: &mut Canvas<T>,
        colors: &ColorMap,
    ) {
        // println!("{:?}", self.active_states);
        // println!("{:?}", self.states_data);
        for i in 0..STATES_COUNT {
            if self.active_states[i] {
                if let Some(data) = self.states_data[i] {
                    if data.draw_at_front != at_front {
                        continue;
                    }
                    data.draw(pos, canvas, colors);
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DisplayData {
    draw_at_front: bool,
    color: ColorTag,
    sub: Option<ColorTag>,
    sub_alfa: f32,
    edge_radius: u8,
    border: Option<Border>,
}
#[derive(Copy, Clone, Debug)]
struct Border {
    color: ColorTag,
    width: u8,
}
impl DisplayData {
    /// for draw behind childrens
    pub fn new(color: ColorTag) -> Self {
        DisplayData {
            draw_at_front: false,
            color,
            sub: None,
            sub_alfa: 0.0,
            edge_radius: 0,
            border: None,
        }
    }
    /// for draw on top of the childrens
    pub fn at_front(mut self) -> Self {
        self.draw_at_front = true;
        self
    }
    pub fn sub(mut self, sub: ColorTag, alfa: f32) -> Self {
        self.sub = Some(sub);
        self.sub_alfa = alfa;
        self
    }
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
    pub fn radius(mut self, radius: u8) -> Self {
        self.edge_radius = radius;
        self
    }
    fn draw<T: RenderTarget>(&self, pos: XYWH, canvas: &mut Canvas<T>, colors: &ColorMap) {
        let color: Color = colors.get(self.color);
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(pos.x, pos.y, pos.w as u32, pos.h as u32));

        if let Some(sub) = self.sub {
            let mut sub: Color = colors.get(sub);
            sub.a = (sub.a as f32 * self.sub_alfa) as u8;
            canvas.set_draw_color(sub);
            canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            canvas.fill_rect(Rect::new(pos.x, pos.y, pos.w as u32, pos.h as u32));
        }
        // canvas.rect(
        //     XYWH::new(pos.x, pos.y, pos.w, pos.h),
        //     border_color,
        //     border_width,
        // );
    }
}
