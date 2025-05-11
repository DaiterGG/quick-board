#[derive(Debug)]
struct Cursor {}
pub enum CursorType {
    Precise,
    Full,
}
impl Cursor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn change(&mut self) {}
}
