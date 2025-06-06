use std::any::Any;
use std::collections::HashMap;

use super::{
    align_vec::AlignVec,
    color_map::*,
    display_vec::DisplayVec,
    element_vec::ElementVec,
    predefined::{Id32, Predefined},
    predefined_styles::PredefinedStyles,
};

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
        let elements = ElementVec::new(Predefined::init());
        let (aligns, displays, elements_data) = PredefinedStyles::init();
        Self {
            elements,
            aligns,
            displays,
            elements_data,
            colors: ColorMap::new(),
        }
    }
}
