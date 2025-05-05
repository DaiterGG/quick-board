use std::cmp::*;

use sdl2::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct WH {
    pub w: i32,
    pub h: i32,
}
impl WH {
    pub fn new(w: i32, h: i32) -> Self {
        WH { w, h }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}
impl XY {
    pub fn new(x: i32, y: i32) -> Self {
        XY { x, y }
    }
    pub fn transform_from(&self, zoom: f32, offset: XY) -> XY {
        XY {
            x: ((self.x - offset.x) as f32 / zoom) as i32,
            y: ((self.y - offset.y) as f32 / zoom) as i32,
        }
    }
    pub fn transform_into(&self, zoom: f32, offset: XY) -> XY {
        XY {
            x: (self.x as f32 * zoom) as i32 + offset.x,
            y: (self.y as f32 * zoom) as i32 + offset.y,
        }
    }
    pub fn is_within(&self, hitbox: XYWH) -> bool {
        self.x >= hitbox.x
            && self.x < hitbox.x + hitbox.w
            && self.y >= hitbox.y
            && self.y < hitbox.y + hitbox.h
    }
    pub fn get_overlap_const(&self, wh: i32, other: XYWH) -> XYWH {
        let x = max(self.x, other.x);
        let y = max(self.y, other.y);
        XYWH {
            x,
            y,
            w: min(self.x + wh, other.x + other.w) - x,
            h: min(self.y + wh, other.y + other.h) - y,
        }
    }
    pub fn substract(&self, other: XY) -> XY {
        XY {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
/// x, y - left top corner
pub struct XYWH {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl XYWH {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> XYWH {
        XYWH { x, y, w, h }
    }
    // /// fram_start = true, left or top border moves
    // pub fn substact_from_start(&mut self, length: i32, horisontal: bool) {
    //     if horisontal {
    //         self.w -= length;
    //         self.x += length;
    //     } else {
    //         self.h -= length;
    //         self.y += length;
    //     }
    // }
    // pub fn substact_from_end(&mut self, length: i32, horisontal: bool) {
    //     if horisontal {
    //         self.w -= length;
    //     } else {
    //         self.h -= length;
    //     }
    // }
    pub fn to_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w as u32, self.h as u32)
    }
    pub fn transform_into(&self, zoom: f32, offset: XY) -> XYWH {
        XYWH {
            x: (self.x as f32 * zoom) as i32 + offset.x,
            y: (self.y as f32 * zoom) as i32 + offset.y,
            w: (self.w as f32 * zoom) as i32,
            h: (self.h as f32 * zoom) as i32,
        }
    }
    pub fn get_overlap(&self, other: XYWH) -> XYWH {
        let x = max(self.x, other.x);
        let y = max(self.y, other.y);
        XYWH {
            x,
            y,
            w: min(self.x + self.w, other.x + other.w) - x,
            h: min(self.y + self.h, other.y + other.h) - y,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}
