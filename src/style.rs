use std::any::Any;

// potential change to
// Style<T: DisplayType>
// or with duplicate Style {display_absolute, display_block}
pub struct Style {
    display_data: Box<dyn Any>,
}
trait DisplayType: Any {}

impl DisplayType for DisplayAbsolute {}
struct DisplayAbsolute {
    pivot: (i16, i16),
    align_horisontal: AlignRule,
    align_vertical: AlignRule,
    align_data: (i16, i16),
}

impl DisplayType for DisplayBlock {}
struct DisplayBlock {
    align_direction: Direction,
    align_data: i16,
}

pub enum AlignRule {
    Center,
    Right,
    Left,
    Top,
    Bottom,
}

enum Direction {
    Horisontal,
    Vertical,
}

struct StyleBuilder {
    style: Option<Style>,
}
impl StyleBuilder {
    fn new() -> StyleBuilder {
        StyleBuilder { style: None }
    }
    pub fn type_absolute(&mut self) -> &mut StyleBuilder {
        let style = &mut self.style;
        assert!(style.is_some(), "StyleBuilder already has display type");
        *style = Some(Style {
            display_data: Box::new(DisplayAbsolute {
                pivot: (0, 0),
                align_horisontal: AlignRule::Center,
                align_vertical: AlignRule::Center,
                align_data: (0, 0),
            }),
        });
        self
    }
    pub fn type_block(&mut self) -> &mut StyleBuilder {
        let style = &mut self.style;
        assert!(style.is_some(), "StyleBuilder already has display type");
        *style = Some(Style {
            display_data: Box::new(DisplayBlock {
                align_direction: Direction::Horisontal,
                align_data: 0,
            }),
        });
        self
    }
    pub fn absolute_pivot(&mut self, x: i16, y: i16) -> &mut StyleBuilder {
        assert!(self.style.is_none(), "StyleBuilder has no style");
        let style = self.style.as_mut().unwrap();
        match style.display_data.downcast_mut::<DisplayAbsolute>() {
            Some(data) => {
                data.pivot = (x, y);
            }
            None => {
                panic!("Style is not absolute")
            }
        }
        self
    }
    pub fn absolute_align(&mut self, align_horisontal: AlignRule, align_vertical: AlignRule) {
        assert!(self.style.is_none(), "StyleBuilder has no style");
        let style = self.style.as_mut().unwrap();
        match style.display_data.downcast_mut::<DisplayAbsolute>() {
            Some(data) => {
                data.align_horisontal = align_horisontal;
                data.align_vertical = align_vertical;
            }
            None => {
                panic!("Style is not absolute")
            }
        }
    }
}

impl Style {
    //  pub fn apply() -> {

    // }
}
