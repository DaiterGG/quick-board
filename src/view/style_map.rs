use super::{
    style::{AlignDirection, AlignSide, AlignValue, Style},
    ui_builder::{BlockId, Id},
};

const STYLES_COUNT: usize = 256;

const DEFAULT_STYLES: [Style; STYLES_COUNT] = {
    let mut s = [Style::None; STYLES_COUNT];
    // style[Id::SomeName as usize] is better than = [style,other_style]
    s[Id::MainDiv as usize] = Style::block(
        AlignDirection::Horisontal,
        AlignSide::Start,
        AlignValue::Absolute(100),
    );
    s[Id::MainDivHeader as usize] = Style::block(
        AlignDirection::Vertical,
        AlignSide::Start,
        AlignValue::Absolute(30),
    );
    s[Id::MainDivLeftPanel as usize] = Style::block(
        AlignDirection::Horisontal,
        AlignSide::Start,
        AlignValue::Relative(30),
    );
    s[Id::MainDivRightPanel as usize] = Style::block(
        AlignDirection::Horisontal,
        AlignSide::End,
        AlignValue::Relative(25),
    );
    s[Id::ForTest1 as usize] = Style::block(
        AlignDirection::Horisontal,
        AlignSide::Start,
        AlignValue::Relative(40),
    );
    s[Id::ForTest2 as usize] = Style::block(
        AlignDirection::Horisontal,
        AlignSide::Start,
        AlignValue::Relative(100),
    );
    s
};

pub struct StyleMap {
    styles: [Style; STYLES_COUNT],
}

impl StyleMap {
    pub fn new_first() -> Self {
        Self {
            styles: DEFAULT_STYLES,
        }
    }
    pub fn new(custom_styles: Vec<Style>) -> Self {
        let mut stl = StyleMap {
            styles: DEFAULT_STYLES,
        };
        for i in 0..custom_styles.len() {
            stl.styles[i] = custom_styles[i];
        }
        stl
    }
    pub fn get(&self, id: Id) -> Style {
        self.styles[id as usize]
    }
    pub fn get_with_index(&self, index: usize) -> Style {
        self.styles[index]
    }
    pub fn overwrite_with_index(&mut self, style: Style, index: usize) {
        self.styles[index] = style;
    }
}

///for testing
impl Default for StyleMap {
    fn default() -> Self {
        Self {
            styles: DEFAULT_STYLES,
        }
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
