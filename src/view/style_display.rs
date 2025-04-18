use std::i32;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use super::color_map::{ColorMap, ColorTag};
use super::coords::{WH, XY, XYWH};
use super::states::States;
use super::style_map::StyleMap;
use super::ui_element::{UIElement, UIElementTrait};

const STATES_COUNT: usize = 7;
pub enum DisplayState {
    Idle,
    Hovered,
    Pressed,
    Released,
    Hield,
    Active,
    Disabled,
}
#[derive(Copy, Clone)]
/// active_states: current element state
/// states_data: constant settings
pub struct Display {
    active_states: [bool; STATES_COUNT],
    states_data: [Option<DisplayData>; STATES_COUNT],
}
#[derive(Copy, Clone)]
pub struct DisplayData {
    draw_at_front: bool,
    color: ColorTag,
    edge_radius: u8,
    border: Option<Border>,
}
#[derive(Copy, Clone)]
struct Border {
    color: ColorTag,
    width: u8,
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
    pub fn hield(&mut self, data: DisplayData) -> &mut Self {
        self.states_data[DisplayState::Hield as usize] = Some(data);
        self
    }
    pub fn set_active(&mut self, state: DisplayState) {
        self.active_states[state as usize] = true;
    }
    pub fn draw<T: RenderTarget>(
        &self,
        pos: XYWH,
        at_front: bool,
        canvas: &mut Canvas<T>,
        colors: &ColorMap,
    ) {
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
impl DisplayData {
    pub fn bg(color: ColorTag) -> Self {
        DisplayData {
            draw_at_front: false,
            color,
            edge_radius: 0,
            border: None,
        }
    }
    pub fn border(&mut self, border: Border) -> &mut Self {
        self.border = Some(border);
        self
    }
    pub fn radius(&mut self, radius: u8) -> &mut Self {
        self.edge_radius = radius;
        self
    }
    fn draw<T: RenderTarget>(&self, pos: XYWH, canvas: &mut Canvas<T>, colors: &ColorMap) {
        let color = colors.get(self.color);
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(pos.x, pos.y, pos.w as u32, pos.h as u32));
        // canvas.rect(
        //     XYWH::new(pos.x, pos.y, pos.w, pos.h),
        //     border_color,
        //     border_width,
        // );
    }
}
