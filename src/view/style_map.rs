use super::{
    color_map::{ColorMap, ColorTag},
    coords::{WH, XY},
    style_align::{Align, Direction, Side, Value},
    style_display::{Display, DisplayData},
    ui_builder::Id,
};

const STYLES_COUNT: usize = 256;

pub struct StyleMap {
    // used separately, but defined here, for convienience
    styles: [(Align, Option<Display>); STYLES_COUNT],

    pub colors: ColorMap,
}

impl StyleMap {
    pub fn new() -> Self {
        let mut m = [(Align::None, None); STYLES_COUNT];
        // root elements always absolute
        m[Id::MainDiv as usize] = (
            Align::absolute(
                XY::new(50, 50),
                XY::new(50, 50),
                (Value::Persent(100), Value::Persent(100)),
            ),
            None,
        );
        m[Id::MainDivHeader as usize] = (
            Align::block(Direction::Vertical, Side::Start, Value::Pixels(50)),
            Some(Display::new(DisplayData::bg(ColorTag::Main))),
        );
        m[Id::MainDivLeftPanel as usize] = (
            Align::block(Direction::Horisontal, Side::Start, Value::Pixels(50)),
            Some(Display::new(DisplayData::bg(ColorTag::Main))),
        );
        m[Id::SoftBorder1 as usize] = (
            Align::block(Direction::Vertical, Side::Start, Value::Pixels(20)),
            Some(Display::new(DisplayData::bg(ColorTag::MainDark))),
        );
        m[Id::LeftBody as usize] = (
            Align::block(Direction::Horisontal, Side::Start, Value::Persent(100)),
            None,
        );
        m[Id::MainDivRightPanel as usize] = (
            Align::block(Direction::Horisontal, Side::End, Value::Persent(25)),
            None,
        );

        m[Id::ButtonTest as usize] = (
            Align::absolute(
                XY::new(50, 50),
                XY::new(50, 50),
                (Value::Pixels(300), Value::Pixels(150)),
            ),
            Some(
                *Display::new(DisplayData::bg(ColorTag::Main))
                    .hovered(DisplayData::bg(ColorTag::SubHover))
                    .pressed(DisplayData::bg(ColorTag::SubHover))
                    .held(DisplayData::bg(ColorTag::SubClick))
                    .released(DisplayData::bg(ColorTag::Main)),
            ),
        );
        m[Id::ForTest1 as usize] = (
            Align::block(Direction::Horisontal, Side::Start, Value::Persent(40)),
            None,
        );
        m[Id::ForTest2 as usize] = (
            Align::block(Direction::Horisontal, Side::Start, Value::Persent(100)),
            None,
        );
        Self {
            styles: m,
            colors: ColorMap::new(),
        }
    }
    pub fn get_align(&self, id: Id) -> Align {
        self.styles[id as usize].0
    }
    pub fn get_align_with_index(&self, index: usize) -> Align {
        self.styles[index].0
    }
    pub fn overwrite_with_index(&mut self, align: Align, display: Option<Display>, index: usize) {
        self.styles[index] = (align, display);
    }

    pub fn get_display(&self, id: Id) -> &Option<Display> {
        &self.styles[id as usize].1
    }
    pub fn get_display_mut(&mut self, id: Id) -> &mut Option<Display> {
        &mut self.styles[id as usize].1
    }
    pub fn get_display_with_index(&self, index: usize) -> Option<Display> {
        self.styles[index].1
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
