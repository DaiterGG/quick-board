use super::{
    element_map::ElementMap,
    predefined::{ID_COUNT, Predefined},
    predefined_styles::PredefinedStyles,
};

use super::{
    color_map::{ColorMap, ColorTag},
    coords::XY,
    predefined::{Id, IdUsize},
    style_align::{Align, Direction, Side, Value},
    style_display::{Display, DisplayData as data},
    ui_element::UIElement,
};

pub struct UIMap {
    elements: Vec<UIElement>,

    // used separately, but defined here, for convienience
    // styles: Vec<(Align, Option<Display>)>,
    aligns: Vec<Align>,
    displays: Vec<Option<Display>>,

    pub colors: ColorMap,
}

impl UIMap {
    pub fn new() -> Self {
        let (aligns, displays) = PredefinedStyles::new();
        Self {
            elements: ElementMap::init(),
            aligns,
            displays,
            colors: ColorMap::new(),
        }
    }
    pub fn align(&self, index: usize) -> Align {
        self.aligns[index]
    }
    pub fn set_align(&mut self, align: Align, index: IdUsize) {
        self.aligns[index] = align;
    }

    pub fn display(&self, id: IdUsize) -> &Option<Display> {
        &self.displays[id]
    }
    pub fn display_mut(&mut self, id: IdUsize) -> &mut Option<Display> {
        &mut self.displays[id]
    }
    pub fn element(&self, id: IdUsize) -> &UIElement {
        &self.elements[id]
    }
    pub fn element_mut(&mut self, id: IdUsize) -> &mut UIElement {
        &mut self.elements[id]
    }
}

///for testing
impl Default for UIMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // struct Div {
    //     def: Def,
    // }
    // #[test]
    // fn it_works() {
    //     let mut div = Div {
    //         def: Def::MainMiddleStyle,
    //     };
    // }
}
