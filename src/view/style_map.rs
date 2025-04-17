use super::{
    style_align::{Align, AlignDirection, AlignSide, AlignValue},
    style_display::Display,
    ui_builder::Id,
};

const STYLES_COUNT: usize = 256;

pub struct StyleMap {
    align: [Align; STYLES_COUNT],
    display: [Display; STYLES_COUNT],
}

impl StyleMap {
    pub fn new() -> Self {
        let mut a = [Align::None; STYLES_COUNT];
        let mut d = [Display::None; STYLES_COUNT];
        // style[Id::SomeName as usize] is better than = [style,other_style]
        a[Id::MainDiv as usize] = Align::block(
            AlignDirection::Horisontal,
            AlignSide::Start,
            AlignValue::Absolute(100),
        );
        a[Id::MainDivHeader as usize] = Align::block(
            AlignDirection::Vertical,
            AlignSide::Start,
            AlignValue::Absolute(30),
        );
        a[Id::MainDivLeftPanel as usize] = Align::block(
            AlignDirection::Horisontal,
            AlignSide::Start,
            AlignValue::Relative(30),
        );
        a[Id::MainDivRightPanel as usize] = Align::block(
            AlignDirection::Horisontal,
            AlignSide::End,
            AlignValue::Relative(25),
        );
        a[Id::ForTest1 as usize] = Align::block(
            AlignDirection::Horisontal,
            AlignSide::Start,
            AlignValue::Relative(40),
        );
        a[Id::ForTest2 as usize] = Align::block(
            AlignDirection::Horisontal,
            AlignSide::Start,
            AlignValue::Relative(100),
        );
        Self {
            align: a,
            display: d,
        }
    }
    pub fn get_align(&self, id: Id) -> Align {
        self.align[id as usize]
    }
    pub fn get_align_with_index(&self, index: usize) -> Align {
        self.align[index]
    }
    pub fn overwrite_align_with_index(&mut self, style: Align, index: usize) {
        self.align[index] = style;
    }

    pub fn get_display(&self, id: Id) -> Display {
        self.display[id as usize]
    }
    pub fn get_display_with_index(&self, index: usize) -> Display {
        self.display[index]
    }
    pub fn overwrite_display_with_index(&mut self, style: Display, index: usize) {
        self.display[index] = style;
    }
}

///for testing
impl Default for StyleMap {
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
    //         def: Def::MainStyle,
    //     };
    // }
}
