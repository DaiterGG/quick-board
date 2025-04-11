extern crate sdl2;

use crate::style::Style;
use crate::transform::Transform;

use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

struct Div {
    transform: Transform,
    style: Style,
    childrens: Vec<Div>,
}

impl Div {
    fn new(x: i16, y: i16, w: i16, h: i16, style: Style) -> Div {
        Div {
            transform: Transform::new(x, y, w, h),
            style,
            childrens: Vec::new(),
        }
    }
    fn add_child(&mut self, child: Div) {
        self.childrens.push(child);
    }
    fn update(&mut self, transform: Transform) {
        self.transform = transform;
        self.update_childrens();
    }
    fn update_childrens(&mut self) {
        for i in 0..self.childrens.len() {
            let tf: Transform;
            self.childrens[i].update(tf);
        }
    }

    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        for child in &mut self.childrens {
            child.draw_to(canvas);
        }
    }
}
