use palette::{FromColor, Hsv, IntoColor, RgbHue, Srgb};
use sdl2::pixels::Color;

use crate::{d, dl};

pub struct ColorOperations {}
impl ColorOperations {
    pub fn hue_palette() -> [u8; 256 * 4 * 3] {
        let mut res = [255; 256 * 4 * 3];
        let rgb = Srgb::new(1.0, 0.0, 0.0);
        let mut hsv = Hsv::from_color(rgb).into_format::<f32>();
        for i in 0..(256 * 3) {
            hsv.hue = (i as f32 / (3.0 * 256.0) * 360.0).into();
            let color: Srgb = hsv.into_format::<f32>().into_color();
            let color_u8 = color.into_format::<u8>();
            res[i * 4] = color_u8.blue;
            res[i * 4 + 1] = color_u8.green;
            res[i * 4 + 2] = color_u8.red;
        }
        res
    }
    pub fn saturation_palette(color: Color, last_hue: f32) -> [u8; 256 * 4] {
        let mut res = [255; 256 * 4];
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();
        let mut hsv = Hsv::from_color(rgb).into_format::<f32>();
        if hsv.saturation == 0.0 {
            hsv.hue = (last_hue * 360.0).into();
        }
        let mut hsv = hsv.into_format::<u8>();
        for i in 0..=255 {
            hsv.saturation = i as u8;
            let color: Srgb = hsv.into_format::<f32>().into_color();
            let color_u8 = color.into_format::<u8>();
            res[i * 4] = color_u8.blue;
            res[i * 4 + 1] = color_u8.green;
            res[i * 4 + 2] = color_u8.red;
        }
        res
    }
    pub fn value_palette(color: Color, last_hue: f32, last_saturation: f32) -> [u8; 256 * 4] {
        let mut res = [255; 256 * 4];
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();

        let mut hsv = Hsv::from_color(rgb).into_format::<f32>();
        if hsv.value == 0.0 {
            hsv.hue = (last_hue * 360.0).into();
            hsv.saturation = last_saturation;
        }
        let mut hsv = hsv.into_format::<u8>();
        for i in 0..=255 {
            hsv.value = i as u8;
            let color: Srgb = hsv.into_format::<f32>().into_color();
            let color_u8 = color.into_format::<u8>();
            res[i * 4] = color_u8.blue;
            res[i * 4 + 1] = color_u8.green;
            res[i * 4 + 2] = color_u8.red;
        }
        res
    }
    pub fn apply_hue(color: Color, hue: f32) -> Color {
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();
        let mut hsv = Hsv::from_color(rgb);
        hsv.hue = (hue * 360.0).into();
        let color: Srgb = hsv.into_format::<f32>().into_color();
        let color_u8 = color.into_format::<u8>();
        Color::RGB(color_u8.red, color_u8.green, color_u8.blue)
    }
    pub fn get_hue(color: Color) -> f32 {
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();
        let hsv = Hsv::from_color(rgb).into_format::<f32>();
        <RgbHue as Into<f32>>::into(hsv.into_components().0) / 360.0
    }
    pub fn apply_saturation(color: Color, saturation: f32, last_hue: f32) -> Color {
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();
        let mut hsv = Hsv::from_color(rgb);
        hsv.hue = (last_hue * 360.0).into();
        hsv.saturation = saturation;
        let color: Srgb = hsv.into_format::<f32>().into_color();
        let color_u8 = color.into_format::<u8>();
        Color::RGB(color_u8.red, color_u8.green, color_u8.blue)
    }
    pub fn get_saturation(color: Color) -> f32 {
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();
        let hsv = Hsv::from_color(rgb).into_format::<f32>();
        hsv.into_components().1
    }

    pub fn apply_value(get: Color, value: f32) -> Color {
        let rgb = Srgb::new(get.r, get.g, get.b).into_format::<f32>();
        let mut hsv = Hsv::from_color(rgb);
        hsv.value = value;
        let color: Srgb = hsv.into_format::<f32>().into_color();
        let color_u8 = color.into_format::<u8>();
        Color::RGB(color_u8.red, color_u8.green, color_u8.blue)
    }
    pub fn get_value(color: Color) -> f32 {
        let rgb = Srgb::new(color.r, color.g, color.b).into_format::<f32>();
        let hsv = Hsv::from_color(rgb).into_format::<f32>();
        hsv.into_components().2
    }

    pub fn reverse_color(color: Color) -> Color {
        let mut rgb = Srgb::new(color.r, color.g, color.b).into_format::<u8>();
        rgb.red = 255 - rgb.red;
        rgb.green = 255 - rgb.green;
        rgb.blue = 255 - rgb.blue;
        Color::RGB(rgb.red, rgb.green, rgb.blue)
    }
}
