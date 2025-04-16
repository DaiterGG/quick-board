#[derive(Copy, Clone)]
pub struct WH {
    pub w: i32,
    pub h: i32,
}
#[derive(Copy, Clone)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
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
    pub const fn new_const(x: i32, y: i32, w: i32, h: i32) -> XYWH {
        XYWH { x, y, w, h }
    }
    /// fram_start = true, left or top border moves
    pub fn substact_from_start(&mut self, length: i32, horisontal: bool) {
        if horisontal {
            self.w -= length;
            self.x += length;
        } else {
            self.h -= length;
            self.y += length;
        }
    }
    pub fn substact_from_end(&mut self, length: i32, horisontal: bool) {
        if horisontal {
            self.w -= length;
        } else {
            self.h -= length;
        }
    }
}
