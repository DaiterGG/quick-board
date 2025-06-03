use std::cmp::*;

use sdl2::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct WH {
    pub w: i32,
    pub h: i32,
}
impl WH {
    pub fn new(w: i32, h: i32) -> WH {
        WH { w, h }
    }
    pub fn new_one(wh: i32) -> WH {
        WH { w: wh, h: wh }
    }
    pub fn mult_one(&self, mult: i32) -> WH {
        WH {
            w: self.w * mult,
            h: self.h * mult,
        }
    }
    pub fn min_one(&self, by: i32) -> WH {
        WH {
            w: min(self.w, by),
            h: min(self.w, by),
        }
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
    pub fn from_u32_tuple(xy: (u32, u32)) -> XY {
        XY {
            x: xy.0 as i32,
            y: xy.1 as i32,
        }
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
    pub fn substract_one(&self, other: i32) -> XY {
        XY {
            x: self.x - other,
            y: self.y - other,
        }
    }
    pub fn add_one(&self, other: i32) -> XY {
        XY {
            x: self.x + other,
            y: self.y + other,
        }
    }
    pub fn add(&self, other: XY) -> XY {
        XY {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn substract(&self, other: XY) -> XY {
        XY {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn mult_one(&self, other: f32) -> XY {
        XY {
            x: (self.x as f32 * other).round() as i32,
            y: (self.y as f32 * other).round() as i32,
        }
    }
    pub fn divide_one(&self, other: f32) -> XY {
        XY {
            x: (self.x as f32 / other).round() as i32,
            y: (self.y as f32 / other).round() as i32,
        }
    }

    pub fn distance(&self, distance_to: XY) -> f32 {
        (((self.x - distance_to.x) as f64 * (self.x - distance_to.x) as f64
            + (self.y - distance_to.y) as f64 * (self.y - distance_to.y) as f64) as f32)
            .sqrt()
    }
    pub fn bound_between(self, other: XY) -> AABB {
        AABB {
            xa: min(self.x, other.x),
            ya: min(self.y, other.y),
            xb: max(self.x, other.x),
            yb: max(self.y, other.y),
        }
    }
    pub fn to_bound(self) -> AABB {
        AABB {
            xa: self.x,
            ya: self.y,
            xb: self.x,
            yb: self.y,
        }
    }
    pub fn to_tr_one(self, wh: i32) -> XYWH {
        XYWH {
            x: self.x,
            y: self.y,
            w: wh,
            h: wh,
        }
    }
    pub fn to_tr(self, wh: WH) -> XYWH {
        XYWH {
            x: self.x,
            y: self.y,
            w: wh.w,
            h: wh.h,
        }
    }
    pub fn to_f32(self) -> XYF32 {
        XYF32 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct XYF32 {
    pub x: f32,
    pub y: f32,
}
impl XYF32 {
    pub fn new(x: f32, y: f32) -> XYF32 {
        XYF32 { x, y }
    }
    pub fn mult_one(&self, mult: f32) -> XYF32 {
        XYF32 {
            x: self.x * mult,
            y: self.y * mult,
        }
    }
    pub fn mult(&self, mult: XYF32) -> XYF32 {
        XYF32 {
            x: self.x * mult.x,
            y: self.y * mult.y,
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
    pub fn get_overlap(&self, other: WH) -> XYWH {
        let x = max(self.x, 0);
        let y = max(self.y, 0);
        XYWH {
            x,
            y,
            w: max(0, min(self.x + self.w, other.w) - x),
            h: max(0, min(self.y + self.h, other.h) - y),
        }
    }
    pub fn xy(&self) -> XY {
        XY {
            x: self.x,
            y: self.y,
        }
    }
    pub fn wh(&self) -> WH {
        WH {
            w: self.w,
            h: self.h,
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
    pub fn to_bound(&self) -> AABB {
        AABB {
            xa: self.x,
            ya: self.y,
            xb: self.x + self.w,
            yb: self.y + self.h,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub xa: i32,
    pub ya: i32,
    pub xb: i32,
    pub yb: i32,
}
impl AABB {
    pub fn new(xa: i32, ya: i32, xb: i32, yb: i32) -> Self {
        Self { xa, ya, xb, yb }
    }
    pub fn expand_one(&mut self, xy: i32) -> Self {
        Self {
            xa: self.xa - xy,
            ya: self.ya - xy,
            xb: self.xb + xy,
            yb: self.yb + xy,
        }
    }
    pub fn expand(&mut self, x: i32, y: i32) -> Self {
        Self {
            xa: self.xa - x,
            xb: self.xb + x,
            ya: self.ya - y,
            yb: self.yb + y,
        }
    }
    pub fn is_overlaping(&self, other: AABB) -> bool {
        self.xa <= other.xb && self.xb >= other.xa && self.ya <= other.yb && self.yb >= other.ya
    }
}
