use sdl2::pixels::Color;

use crate::{d, dl};

use super::{
    action_pump::*,
    align_vec::AlignVec,
    coords::XY,
    display_vec::DisplayVec,
    predefined::*,
    style_display::DisplayState,
    texture_manager::TextureManager,
    ui_map::{ElemDataMap, UIMap},
};

pub struct Txt {
    current_text: String,
    absolute_scale: u32,
    text_at: Vec<DisplayState>,
    color: Color,
}
impl Txt {
    pub fn new(current_text: String, absolute_scale: u32, text_at: Vec<DisplayState>) -> Self {
        Self {
            current_text,
            absolute_scale,
            text_at,
            color: Color::RGBA(214, 214, 214, 0),
        }
    }
    // pub fn before_collision(id: IdI32, input: &mut InputState, ui: &mut UIMap, hit: bool) {
    //     let this_data = &ui.sliders_data[&id];
    //     let this_tr = ui.elements[id as usize].transform;
    // }
    pub fn update_scale(
        new_ui_size: f32,
        id: Id32,
        ui: &mut UIMap,
        t_manager: &mut TextureManager,
    ) {
        let this = Self::get_self(id, &mut ui.elements_data);
        this.text_texture(id, &mut ui.displays, &mut ui.aligns, t_manager, new_ui_size);
    }
    pub fn update_text(
        new_text: String,
        id: Id32,
        ui: &mut UIMap,
        t_manager: &mut TextureManager,
        ui_size: f32,
    ) {
        let this = Self::get_self(id, &mut ui.elements_data);
        this.current_text = new_text;
        this.text_texture(id, &mut ui.displays, &mut ui.aligns, t_manager, ui_size);
    }
    pub fn text_texture(
        &mut self,
        id: Id32,
        displays: &mut DisplayVec,
        aligns: &mut AlignVec,
        t_manager: &mut TextureManager,
        ui_size: f32,
    ) {
        let display = displays.get_mut_unwrap(id);
        let align = &mut aligns.get_mut(id);
        let actual_scale = (self.absolute_scale as f32 * ui_size) as u16;
        let (tex_id, string_size) =
            t_manager.new_text_texture(&self.current_text, actual_scale, self.color);
        let string_size = XY::from_u32_tuple(string_size).divide_one(ui_size);
        for state in self.text_at.iter() {
            let data = display.states_data[*state as usize].as_mut().unwrap();
            let old = data.tex_id;
            if old.is_some() && old.unwrap() != tex_id {
                t_manager.textures.destroy_texture_data(old.unwrap());
            }
            data.set_tex(tex_id);
            align.set_size(string_size);
            ActionPump::add(Action::UIUpdate);
        }
    }
    fn get_self(id: Id32, data: &mut ElemDataMap) -> &mut Txt {
        data.get_mut(&id)
            .unwrap_or_else(|| panic!("element - '{:?}' does not exist", id))
            .downcast_mut::<Txt>()
            .unwrap_or_else(|| {
                panic!(
                    "element - '{:?}' is subscribed to a callback that expects a txt element",
                    id
                )
            })
    }
}
