use std::any::Any;
use std::collections::HashMap;

use super::align_vec::AlignVec;
use super::display_vec::DisplayVec;
use super::element_vec::ElementVec;
use super::predefined::Id32;
use super::predefined_styles::PredefinedStyles;

use super::color_map::*;

pub type ElemDataMap = HashMap<Id32, Box<dyn Any>>;
pub struct UIMap {
    pub elements: ElementVec,

    pub aligns: AlignVec,
    pub displays: DisplayVec,
    pub elements_data: ElemDataMap,
    pub colors: ColorMap,
}

impl UIMap {
    pub fn new() -> Self {
        let (aligns, displays, elements_data) = PredefinedStyles::init();
        Self {
            elements: ElementVec::init(),
            aligns,
            displays,
            elements_data,
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
