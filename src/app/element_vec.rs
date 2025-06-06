use super::{
    predefined::{Id, Id32},
    ui_element::UIElement,
};

pub struct ElementVec {
    vec: Vec<Option<UIElement>>,
}
impl ElementVec {
    pub fn new(vec: Vec<Option<UIElement>>) -> Self {
        Self { vec }
    }
    pub fn push(&mut self, element: UIElement) {
        self.vec.push(Some(element));
    }
    pub fn get(&self, id: Id32) -> &UIElement {
        self.vec[id.usize()]
            .as_ref()
            .unwrap_or_else(|| panic!("display '{:?}' does not exist", id))
    }
    pub fn get_mut(&mut self, id: Id32) -> &mut UIElement {
        self.vec[id.usize()]
            .as_mut()
            .unwrap_or_else(|| panic!("display '{:?}' does not exist", id))
    }
    pub fn get_from_id(&self, id: Id) -> &UIElement {
        self.vec[id as usize]
            .as_ref()
            .unwrap_or_else(|| panic!("display '{:?}' does not exist", id))
    }
}
