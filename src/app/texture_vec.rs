use std::mem::replace;

use sdl2::{
    pixels::Color,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::app::coords::WH;

use super::{color_operations::ColorOperations, texture_data::TextureData};
use indices::indices;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TexId16(u16);
impl TexId16 {
    fn usize(self) -> usize {
        self.0 as usize
    }
}
impl From<TexId> for TexId16 {
    fn from(val: TexId) -> Self {
        TexId16(val as u16)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TexId {
    IconBrush,
    IconMove,
    IconSample,

    RangeHue,
    RangeValue,
    RangeSaturation,

    Total,
}
pub struct TextureVec {
    vec: Vec<Option<TextureData>>,
    unused_textures: Vec<TexId16>,
}
impl TextureVec {
    pub fn new(t_creator: &TextureCreator<WindowContext>) -> Self {
        let mut p = [const { None }; Total as usize];

        // use PixelFormatEnum::*;
        use TexId::*;
        use TextureData as T;

        p[RangeHue as usize] = T::some(&t_creator, WH::new(256 * 3, 1));
        p[RangeSaturation as usize] = T::some(&t_creator, WH::new(256, 1));
        p[RangeValue as usize] = T::some(&t_creator, WH::new(256, 1));

        p[IconBrush as usize] = T::from_bytes(
            &t_creator,
            include_bytes!("../../resources/icons/brush.png"),
        );
        p[IconMove as usize] = T::from_bytes(
            &t_creator,
            include_bytes!("../../resources/icons/move_tool.png"),
        );
        p[IconSample as usize] = T::from_bytes(
            &t_creator,
            include_bytes!("../../resources/icons/sample.png"),
        );
        let vec = Vec::from(p);
        Self {
            vec,
            unused_textures: Vec::new(),
        }
    }
    pub fn push(&mut self, display: TextureData) -> TexId16 {
        self.vec.push(Some(display));
        TexId16(self.vec.len() as u16 - 1)
    }
    pub fn get(&self, id: TexId16) -> &TextureData {
        self.vec[id.usize()]
            .as_ref()
            .unwrap_or_else(|| panic!("texture data '{:?}' does not exist", id))
    }
    pub fn get_mut(&mut self, id: TexId16) -> &mut TextureData {
        self.vec[id.usize()]
            .as_mut()
            .unwrap_or_else(|| panic!("texture data '{:?}' does not exist", id))
    }
    pub fn get_mut_3(
        &mut self,
        one_id: TexId16,
        two_id: TexId16,
        three_id: TexId16,
    ) -> (&mut TextureData, &mut TextureData, &mut TextureData) {
        let (one, two, three) = indices!(
            &mut self.vec,
            one_id.usize(),
            two_id.usize(),
            three_id.usize()
        );
        (
            one.as_mut().unwrap(),
            two.as_mut().unwrap(),
            three.as_mut().unwrap(),
        )
    }
    pub fn init_texture(&mut self, texture_data: TextureData) -> TexId16 {
        if !self.unused_textures.is_empty() {
            let id = self.unused_textures.pop().unwrap();
            self.vec[id.usize()] = Some(texture_data);
            return id;
        }
        self.vec.push(Some(texture_data));
        TexId16(self.vec.len() as u16 - 1)
    }
    pub fn destroy_texture_data(&mut self, id: TexId16) {
        unsafe {
            self.unused_textures.push(id);
            let data = self.vec[id.usize()].take();
            if let Some(data) = data {
                data.texture.destroy();
            }
        }
    }
    pub fn replace_texture<T: Into<usize>>(&mut self, texture: Texture, id: T) {
        unsafe {
            let id = id.into();
            let old = replace(
                &mut self.vec[id]
                    .as_mut()
                    .expect("Texture to replace not found")
                    .texture,
                texture,
            );
            old.destroy();
        }
    }
    pub fn replace_texture_data<T: Into<usize>>(&mut self, texture_data: TextureData, id: T) {
        let id = id.into();
        let old = replace(&mut self.vec[id], Some(texture_data));
        unsafe {
            if let Some(old) = old {
                old.texture.destroy();
            }
        }
    }
    // static palettes
    pub fn init_palettes(&mut self) {
        self.get_mut(TexId::RangeHue.into())
            .texture
            .update(None, &ColorOperations::hue_palette(), 256 * 4 * 3)
            .unwrap();
    }
    // dynamic palettes
    pub fn update_palettes(&mut self, color: Color, last_hue: f32, last_saturation: f32) {
        self.get_mut(TexId::RangeSaturation.into())
            .texture
            .update(
                None,
                &ColorOperations::saturation_palette(color, last_hue),
                256 * 4,
            )
            .unwrap();
        self.get_mut(TexId::RangeValue.into())
            .texture
            .update(
                None,
                &ColorOperations::value_palette(color, last_hue, last_saturation),
                256 * 4,
            )
            .unwrap();
    }
}
