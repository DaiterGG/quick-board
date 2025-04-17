extern crate sdl2;

use super::states::States;
use super::ui_element::{UIElement, UIElementTrait};

pub struct Div;
impl UIElementTrait for Div {
    fn pointer_collision(&self, states: &mut States) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::view::{
        coords::{WH, XYWH},
        style_align::Align,
        style_map::StyleMap,
        ui_builder::Id,
        ui_element::ElementType,
    };

    use super::*;

    #[test]
    pub fn fit() {
        let win = XYWH::new(0, 0, 1000, 1000);
        let mut childs = vec![UIElement::new(
            Id::ForTest1,
            ElementType::Div(Div {}),
            vec![],
        )];
        Align::fit_childrens(
            win,
            &mut childs,
            &StyleMap::new(),
            &mut States::new(WH { w: 1000, h: 1000 }),
        );
        assert_eq!(childs[0].transform.x, 0);
        assert_eq!(childs[0].transform.w, 400);
        assert_eq!(childs[1].transform.x, 400);
        assert_eq!(childs[1].transform.w, 600);
    }
}
