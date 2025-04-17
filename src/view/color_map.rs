const COLOR_COUNT: usize = 256;

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
        let mut c = [Color::bl(); COLOR_COUNT];
        c[ColorTag::Deep as usize] = Color::new(0, 0, 0);
        c[ColorTag::Main as usize] = Color::new(255, 255, 255);
        c[ColorTag::MainLight as usize] = Color::new(255, 255, 255);
        c[ColorTag::MainDark as usize] = Color::new(255, 255, 255);
        ColorMap { colors: c }
    }
    pub fn get(&self, tag: ColorTag) -> Color {
        self.colors[tag as usize]
    }
    pub fn get_by_id(&self, id: usize) -> Color {
        self.colors[id]
    }
}
#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub fn bl() -> Color {
        Color::new(0, 0, 0)
    }
}
