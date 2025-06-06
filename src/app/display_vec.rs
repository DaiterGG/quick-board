use super::{
    predefined::{Id, Id32},
    style_display::Display,
};

pub struct DisplayVec {
    vec: Vec<Option<Display>>,
}
impl DisplayVec {
    pub fn new(vec: Vec<Option<Display>>) -> Self {
        Self { vec }
    }
    pub fn push(&mut self, display: Display) {
        self.vec.push(Some(display));
    }
    /// returns opiton bc element can have no display
    /// * `id`:
    pub fn get(&self, id: Id32) -> Option<&Display> {
        self.vec[id.usize()].as_ref()
    }
    pub fn get_unwrap(&self, id: Id32) -> &Display {
        self.vec[id.usize()]
            .as_ref()
            .unwrap_or_else(|| panic!("display '{:?}' does not exist", id))
    }
    pub fn get_mut(&mut self, id: Id32) -> Option<&mut Display> {
        self.vec[id.usize()].as_mut()
    }
    pub fn get_mut_unwrap(&mut self, id: Id32) -> &mut Display {
        self.vec[id.usize()]
            .as_mut()
            .unwrap_or_else(|| panic!("display '{:?}' does not exist", id))
    }
    pub fn get_from_id(&self, id: Id) -> Option<&Display> {
        self.vec[id as usize].as_ref()
    }
}
