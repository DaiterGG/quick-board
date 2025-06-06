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
    Blue,
    Green,
    BorderDark,
    CurrentColor,
    CurrentColorReverse,
    Total,
}

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
        c[ColorTag::BorderDark as usize] = Color::RGB(20, 20, 20);
        c[ColorTag::Red as usize] = Color::RGB(255, 0, 0);
        c[ColorTag::Blue as usize] = Color::RGB(0, 0, 255);
        c[ColorTag::Green as usize] = Color::RGB(0, 255, 0);
        c[ColorTag::CurrentColor as usize] = Color::RGB(0, 0, 0);
        c[ColorTag::CurrentColorReverse as usize] = Color::RGB(0, 0, 0);
        ColorMap { colors: c }
    }
    pub fn set(&mut self, tag: ColorTag, color: Color) {
        self.colors[tag as usize] = color;
    }
    pub fn get(&self, tag: ColorTag) -> Color {
        self.colors[tag as usize]
    }
    pub fn get_mut(&mut self, tag: ColorTag) -> &mut Color {
        &mut self.colors[tag as usize]
    }
}
