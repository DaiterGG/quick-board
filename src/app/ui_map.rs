use std::collections::HashMap;

use super::predefined::IdI32;
use super::slider::Slider;
use super::{element_map::ElementMap, predefined_styles::PredefinedStyles};

use super::{color_map::*, style_align::*, style_display::*, ui_element::UIElement};

pub struct UIMap {
    pub elements: Vec<UIElement>,

    // used separately, but defined here, for convienience
    // styles: Vec<(Align, Option<Display>)>,
    pub aligns: Vec<Align>,
    pub displays: Vec<Option<Display>>,
    pub sliders_data: HashMap<IdI32, Slider>,

    pub colors: ColorMap,
}

impl UIMap {
    pub fn new() -> Self {
        let (aligns, displays, sliders_data) = PredefinedStyles::init();
        Self {
            elements: ElementMap::init(),
            aligns,
            displays,
            sliders_data,
            colors: ColorMap::new(),
        }
    }
}

#[cfg(test)]
impl Default for UIMap {
    fn default() -> Self {
        Self::new()
    }
}
