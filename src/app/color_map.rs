use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorTag {
    Deep,
    MainMiddle,
    MainLight,
    MainDark,
    Sub,
    FlashClick,
    Red,
    Total,
}
pub const COLOR_COUNT: usize = ColorTag::Total as usize;

pub struct ColorMap {
    colors: [Color; ColorTag::Total as usize],
}
impl ColorMap {
    pub fn new() -> ColorMap {
        let mut c = [Color::RGB(0, 0, 0); ColorTag::Total as usize];
        c[ColorTag::Deep as usize] = Color::RGB(14, 14, 14);
        c[ColorTag::MainDark as usize] = Color::RGB(25, 25, 25);
        c[ColorTag::MainMiddle as usize] = Color::RGB(31, 31, 31);
        c[ColorTag::MainLight as usize] = Color::RGB(36, 36, 36);
        c[ColorTag::Sub as usize] = Color::RGB(0, 159, 177);
        c[ColorTag::FlashClick as usize] = Color::RGB(255, 255, 255);
        c[ColorTag::Red as usize] = Color::RGB(255, 0, 0);
        ColorMap { colors: c }
    }
    pub fn get(&self, tag: ColorTag) -> Color {
        self.colors[tag as usize]
    }
    pub fn get_by_id(&self, id: usize) -> Color {
        self.colors[id]
    }
}
