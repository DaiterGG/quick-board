use super::{element_map::ElementMap, predefined_styles::PredefinedStyles};

use super::{color_map::*, style_align::*, style_display::*, ui_element::UIElement};

pub struct UIMap {
    pub elements: Vec<UIElement>,

    // used separately, but defined here, for convienience
    // styles: Vec<(Align, Option<Display>)>,
    pub aligns: Vec<Align>,
    pub displays: Vec<Option<Display>>,

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
