pub struct Transform {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

impl Transform {
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Transform {
        Transform { x, y, w, h }
    }
}
