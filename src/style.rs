// potential change to
// display_data: Box<dyn Any>,
// Style<T: DisplayType>,
// or with duplicate Style {display_absolute, display_block}
trait DisplayType {}
pub struct Style<T: DisplayType> {
    display_data: T,
}

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

struct StyleBuilder<T: DisplayType> {
    style: &mut Style<T>,
}
impl StyleBuilder<DisplayAbsolute> {
    fn new() -> StyleBuilder<DisplayAbsolute> {
        StyleBuilder {
            style: Style::<DisplayAbsolute> {
                display_data: DisplayAbsolute {
                    pivot: (0, 0),
                    align_horisontal: AlignRule::Center,
                    align_vertical: AlignRule::Center,
                    align_data: (0, 0),
                },
            },
        }
    }
    pub fn pivot(&mut self, x: i16, y: i16) -> &mut StyleBuilder<DisplayAbsolute> {
        self.style.display_data.pivot = (x, y);
        self
    }
    pub fn absolute_align(
        &mut self,
        align_horisontal: AlignRule,
        align_vertical: AlignRule,
    ) -> &mut StyleBuilder<DisplayAbsolute> {
        self.style.display_data.align_horisontal = align_horisontal;
        self.style.display_data.align_vertical = align_vertical;
        self
    }
    pub fn align_data(&mut self, x: i16, y: i16) -> &mut StyleBuilder<DisplayAbsolute> {
        self.style.display_data.align_data = (x, y);
        self
    }
}
impl StyleBuilder<DisplayBlock> {
    fn new() -> StyleBuilder<DisplayBlock> {
        StyleBuilder {
            style: Style::<DisplayBlock> {
                display_data: DisplayBlock {
                    align_direction: Direction::Horisontal,
                    align_data: 0,
                },
            },
        }
    }
    pub fn align_data(&mut self, data: i16) -> &mut StyleBuilder<DisplayBlock> {
        self.style.display_data.align_data = data;
        self
    }
    pub fn align_horisontal(&mut self) -> &mut StyleBuilder<DisplayBlock> {
        self.style.display_data.align_direction = Direction::Horisontal;
        self
    }
    pub fn align_direction(&mut self) -> &mut StyleBuilder<DisplayBlock> {
        self.style.display_data.align_direction = Direction::Vertical;
        self
    }
    pub fn build(&self) -> &mut Style<DisplayBlock> {
        &mut self.style
    }
}

impl Style<DisplayAbsolute> {
    //  pub fn apply() -> {

    // }
}
