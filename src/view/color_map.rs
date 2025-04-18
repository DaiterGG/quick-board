use sdl2::pixels::Color;

const COLOR_COUNT: usize = 256;

#[derive(Copy, Clone)]
pub enum ColorTag {
    Deep,
    Main,
    MainLight,
    MainDark,
}
pub struct ColorMap {
    colors: [Color; COLOR_COUNT],
}
impl ColorMap {
    pub fn new() -> ColorMap {
        let mut c = [Color::RGB(0, 0, 0); COLOR_COUNT];
        c[ColorTag::Deep as usize] = Color::RGB(14, 14, 14);
        c[ColorTag::MainDark as usize] = Color::RGB(25, 25, 25);
        c[ColorTag::Main as usize] = Color::RGB(31, 31, 31);
        c[ColorTag::MainLight as usize] = Color::RGB(36, 36, 36);
        ColorMap { colors: c }
    }
    pub fn get(&self, tag: ColorTag) -> Color {
        self.colors[tag as usize]
    }
    pub fn get_by_id(&self, id: usize) -> Color {
        self.colors[id]
    }
}
