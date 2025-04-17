pub struct Button {}
impl Button {
    pub const fn new(id: Id, bg_color: Option<Color>) -> Div {
        Div {
            id,
            current_transform: XYWH::new_const(0, 0, 0, 0),
            bg_color,
            childrens,
        }
    }
}
impl UIElement for button {
    fn update_pos(&mut self, transform: XYWH, styles: &StyleMap, states: &mut States) {
        self.current_transform = transform;
        Style::fit_childrens(self.current_transform, &mut self.childrens, styles, states);
    }
    fn pointer_collision(&self, states: &mut States) {
        todo!()
    }

    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        if let Some(background) = &self.bg_color {
            canvas.set_draw_color(*background);
            canvas
                .fill_rect(Rect::new(
                    self.current_transform.x as i32,
                    self.current_transform.y as i32,
                    self.current_transform.w as u32,
                    self.current_transform.h as u32,
                ))
                .unwrap();
        }
        for i in (0..self.childrens.len()).rev() {
            self.childrens[i].draw_to(canvas);
        }
    }
    fn get_id(&self) -> Id {
        self.id
    }
}
