use crate::app::{
    predefined::{Id, Id32},
    style_align::Align,
};

pub struct AlignVec {
    vec: Vec<Option<Align>>,
}
impl AlignVec {
    pub fn new(vec: Vec<Option<Align>>) -> Self {
        Self { vec }
    }
    pub fn push(&mut self, display: Align) {
        self.vec.push(Some(display));
    }
    pub fn get(&self, id: Id32) -> &Align {
        self.vec[id.usize()]
            .as_ref()
            .unwrap_or_else(|| panic!("align '{:?}' does not exist", id))
    }
    pub fn get_mut(&mut self, id: Id32) -> &mut Align {
        self.vec[id.usize()]
            .as_mut()
            .unwrap_or_else(|| panic!("align '{:?}' does not exist", id))
    }
    pub fn get_from_id(&self, id: Id) -> &Align {
        self.vec[id as usize]
            .as_ref()
            .unwrap_or_else(|| panic!("align '{:?}' does not exist", id))
    }
}
