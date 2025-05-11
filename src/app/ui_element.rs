use sdl2::{render::*, video::Window};

use super::{coords::XYWH, predefined::*, texture_manager::TextureManager, ui_map::UIMap};

pub struct UIElement {
    pub element_type: ElementType,
    pub id: IdI32,
    pub childrens: Vec<IdI32>,
    pub transform: XYWH,
}
#[derive(Copy, Clone)]
pub enum ElementType {
    Div,
    Button,
    DrawWindow,
}

impl UIElement {
    pub fn new(element: ElementType, id: IdI32, childrens: Vec<IdI32>) -> Self {
        Self {
            element_type: element,
            id,
            childrens,
            transform: XYWH::zero(),
        }
    }
    pub fn draw_to(&self, canvas: &mut Canvas<Window>, styles: &UIMap, textures: &TextureManager) {
        let dis = &styles.displays[self.id as usize];
        let color = &styles.colors;

        dis.as_ref()
            .inspect(|d| d.draw(self.transform, false, canvas, color, textures));

        for i in 0..self.childrens.len() {
            styles.elements[self.childrens[i] as usize].draw_to(canvas, styles, textures);
        }

        dis.as_ref()
            .inspect(|d| d.draw(self.transform, true, canvas, color, textures));
    }
}
// #[cfg(test)]
// impl Default for UIElement {
//     fn default() -> Self {
//         Self {
//             element_type: ElementType::Div,
//             id: Id::ForTest1 as usize,
//             childrens: Vec::new(),
//             transform: XYWH::default(),
//         }
//     }
// }
// #[cfg(test)]
// mod tests {
//     use crate::view::{coords::XYWH, ui_element::ElementType};

//     use super::*;

//     #[test]
//     pub fn fit() {
//         let win = XYWH::new(0, 0, 1000, 1000);
//         let mut childs = vec![
//             UIElement::new(Id::ForTest1 as usize, ElementType::Div, vec![]),
//             UIElement::new(Id::ForTest2 as usize, ElementType::Div, vec![]),
//         ];
//         let mut div = UIElement::new(Id::ForTest1, ElementType::Div, childs);
//         UIElement::update(
//             Id::ForTest1 as usize,
//             win,
//             &mut UIMap::new(),
//             &mut States::default(),
//         );
//         assert_eq!(div.childrens[0].transform.x, 0);
//         assert_eq!(div.childrens[0].transform.w, 400);
//         assert_eq!(div.childrens[1].transform.x, 400);
//         assert_eq!(div.childrens[1].transform.w, 600);
//     }
// }
