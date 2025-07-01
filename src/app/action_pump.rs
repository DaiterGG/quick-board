use sdl2::pixels::Color;

use super::{
    canvas_manager::CanvasManager, color_map::ColorTag, color_operations::ColorOperations,
    coords::*, input_state::InputState, predefined::*, slider::Slider,
    texture_manager::TextureManager, tool_trait::ToolId, txt::Txt, ui_manager::UIManager,
    ui_map::UIMap,
};

use std::collections::HashMap;
use std::mem::take;
use std::sync::{Mutex, OnceLock};

#[derive(Copy, Clone)]
pub enum Action {
    ButtonPressed(Id32),
    HoldTool(ToolId, bool),
    ChangeTool(ToolId),
    Undo,
    Redo,
    Drag(Id32, XYF32),
    DragEnd(Id32),
    SliderLine(Id32, f32),
    SliderCoord(Id32, XYF32),
    BrushSize(bool),
    CursorInCanvas(bool),
    UIUpdate,
    WindowResized,
    CanvasSlide,
    ColorFullUpdate(Color),

    //observers
    ColorObserve(Color),
    UISizeObserve(f32),
    ToolSizeObserve(f32),
    BrushDensityObserve(i32),
    BrushHardnessObserve(i32),
    BrushAlfaObserve(i32),
}
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Callback {
    HSVHue,
    HSVSaturation,
    HSVValue,
    TxtScale,
    ToolSizeTxt,
    BrushHardnessTxt,
    BrushDensityTxt,
    BrushAlfaTxt,
}

pub static A_PUMP: OnceLock<ActionPump> = OnceLock::new();

/// # Static queue that can execute code and parse backend data to display it
///
/// `actions`: list of predefined code
/// `callback_data`: data to update ui elements, in case if backend data changed
/// it is bound to predefined Ids -> cannot be created dynamically
/// ui elements can store their own, or a copy of Id to access updated data
pub struct ActionPump {
    actions: Mutex<Vec<Action>>,
    callbacks: Mutex<HashMap<Callback, Vec<Id32>>>,
}
impl ActionPump {
    pub fn init() {
        A_PUMP
            .set(ActionPump {
                actions: Mutex::new(Vec::new()),
                callbacks: Mutex::new(HashMap::new()),
            })
            .unwrap_or_else(|_| panic!("OnceLock init error"))
    }
    pub fn subscribe_to_callback(id: Id32, callback: Callback) {
        let pump = A_PUMP.get().unwrap();
        let mut callbacks = pump.callbacks.lock().unwrap();
        match callbacks.get_mut(&callback) {
            Some(ids) => {
                ids.push(id);
            }
            None => {
                callbacks.insert(callback, vec![id]);
            }
        }
    }

    pub fn add(action: Action) {
        A_PUMP.get().unwrap().actions.lock().unwrap().push(action);
    }
    pub fn apply(
        c_manager: &mut CanvasManager,
        ui: &mut UIManager,
        ui_map: &mut UIMap,
        input: &mut InputState,
        t_manager: &mut TextureManager,
        delta: f32,
        frame_count: u32,
    ) {
        let this = A_PUMP.get().unwrap();
        let actions = take(&mut (*this.actions.lock().unwrap()));
        let callbacks = &mut *this.callbacks.lock().unwrap();
        use Action::*;
        use Callback::*;
        for action in actions {
            match action {
                ButtonPressed(id) if id == Id::BrushButton.into() => {
                    c_manager.change_tool(ToolId::Brush);
                }
                ButtonPressed(id) if id == Id::MoveButton.into() => {
                    c_manager.change_tool(ToolId::Move);
                }
                ButtonPressed(id) if id == Id::SampleButton.into() => {
                    c_manager.change_tool(ToolId::Sample);
                }
                ButtonPressed(id) if id == Id::BrushEraseCheck.into() => {
                    c_manager.tools.brush.erase_mode = !c_manager.tools.brush.erase_mode;
                }
                Drag(id, delta) if id == Id::ToolSizeDrag.into() && delta.x != 0.0 => {
                    const CONST: f32 = 0.5;
                    let d_x = delta.x * CONST * ui.ui_scale.get();
                    c_manager.add_tool_size(d_x, t_manager);
                    input.cursor.set_active(true);
                }
                Drag(id, delta) if id == Id::BrushHardnessDrag.into() && delta.x != 0.0 => {
                    const CONST: f32 = 0.5;

                    // TODO: add sub_f32 var
                    let val = if delta.x > 0.0 { 1 } else { -1 }
                        * (delta.x * CONST * ui.ui_scale.get()).abs().ceil() as i32;
                    let alfa = &mut c_manager.tools.brush.alfa_hardness;
                    alfa.set((alfa.get() + val).clamp(1, 100));
                }
                Drag(id, delta) if id == Id::BrushDensityDrag.into() && delta.x != 0.0 => {
                    const CONST: f32 = 0.5;
                    let val = if delta.x > 0.0 { 1 } else { -1 }
                        * (delta.x * CONST * ui.ui_scale.get()).abs().ceil() as i32;
                    let den = &mut c_manager.tools.brush.draw_density;
                    den.set((den.get() + val).clamp(1, 800));
                }
                Drag(id, delta) if id == Id::BrushAlfaDrag.into() && delta.x != 0.0 => {
                    const CONST: f32 = 0.5;
                    let val = if delta.x > 0.0 { 1 } else { -1 }
                        * (delta.x * CONST * ui.ui_scale.get()).abs().ceil() as i32;
                    let alfa = &mut c_manager.tools.brush.alfa;
                    alfa.set((alfa.get() + val).clamp(1, 100));
                }
                DragEnd(id) if id == Id::BrushHardnessDrag.into() => {
                    c_manager.tools.brush.generate_circle_alfa_mask(t_manager);
                    c_manager.tools.brush.update_buffer(t_manager);
                }
                HoldTool(id, hold_in) => c_manager.try_hold_tool(id, hold_in),
                ChangeTool(tool_id) => c_manager.change_tool(tool_id),
                Undo => c_manager.data.history.undo(t_manager),
                Redo => c_manager.data.history.redo(),
                BrushSize(increase) => {
                    c_manager.mult_tool_size(increase, t_manager);
                }
                CursorInCanvas(in_canvas) => {
                    input.cursor.set_active(in_canvas);
                }
                SliderLine(id, ratio) if id == Id::HSV_H.into() => {
                    let color = ColorOperations::apply_hue(c_manager.data.color.get(), ratio);
                    c_manager.data.last_hue = ratio;
                    c_manager.data.color.set(color);
                }
                SliderLine(id, ratio) if id == Id::HSV_S.into() => {
                    let color = ColorOperations::apply_saturation(
                        c_manager.data.color.get(),
                        ratio,
                        c_manager.data.last_hue,
                    );
                    c_manager.data.last_saturation = ratio;
                    c_manager.data.color.set(color);
                }
                SliderLine(id, ratio) if id == Id::HSV_V.into() => {
                    let old_val = ColorOperations::get_value(c_manager.data.color.get());
                    let mut color = ColorOperations::apply_value(c_manager.data.color.get(), ratio);

                    // TODO: need better handling
                    if old_val == 0.0 {
                        color = ColorOperations::apply_saturation(
                            color,
                            c_manager.data.last_saturation,
                            c_manager.data.last_hue,
                        );
                        color = ColorOperations::apply_hue(color, c_manager.data.last_hue);
                    }

                    c_manager.data.color.set(color);
                }
                UIUpdate => {
                    ui.requires_update = true;
                }
                WindowResized => {
                    c_manager.tools.brush.generate_circle_mask(t_manager);
                    c_manager.tools.brush.generate_circle_alfa_mask(t_manager);
                    c_manager.tools.brush.update_buffer(t_manager);
                    ui.requires_update = true;
                }

                ColorFullUpdate(color) => {
                    t_manager.textures.update_palettes(
                        color,
                        c_manager.data.last_hue,
                        c_manager.data.last_saturation,
                    );

                    for id in callbacks.get(&HSVHue).unwrap_or(&vec![]) {
                        Slider::update(ColorOperations::get_hue(color), *id, ui_map);
                    }
                    for id in callbacks.get(&HSVSaturation).unwrap_or(&vec![]) {
                        Slider::update(ColorOperations::get_saturation(color), *id, ui_map);
                    }
                    for id in callbacks.get(&HSVValue).unwrap_or(&vec![]) {
                        Slider::update(ColorOperations::get_value(color), *id, ui_map);
                    }
                }
                ColorObserve(color) => {
                    if color.r != color.b || color.r != color.g || color.b != color.g {
                        c_manager.data.last_hue = ColorOperations::get_hue(color);
                        c_manager.data.last_saturation = ColorOperations::get_saturation(color);
                    }
                    t_manager.textures.update_palettes(
                        color,
                        c_manager.data.last_hue,
                        c_manager.data.last_saturation,
                    );
                    ui_map.colors.set(ColorTag::CurrentColor, color);
                    ui_map.colors.set(
                        ColorTag::CurrentColorReverse,
                        ColorOperations::reverse_color(color),
                    );
                }
                UISizeObserve(size) => {
                    for id in callbacks.get(&TxtScale).unwrap_or(&vec![]) {
                        Txt::update_scale(size, *id, ui_map, t_manager);
                    }
                }
                ToolSizeObserve(_sub_size) => {
                    for id in callbacks.get(&ToolSizeTxt).unwrap_or(&vec![]) {
                        Txt::update_text(
                            format!("{}", c_manager.tools.get_size(c_manager.current_tool)),
                            *id,
                            ui_map,
                            t_manager,
                            ui.ui_scale.get(),
                        );
                    }
                }
                BrushHardnessObserve(hardness) => {
                    for id in callbacks.get(&BrushHardnessTxt).unwrap_or(&vec![]) {
                        Txt::update_text(
                            format!("{}", hardness),
                            *id,
                            ui_map,
                            t_manager,
                            ui.ui_scale.get(),
                        );
                    }
                }
                BrushDensityObserve(draw_density) => {
                    for id in callbacks.get(&BrushDensityTxt).unwrap_or(&vec![]) {
                        Txt::update_text(
                            format!("{}", draw_density),
                            *id,
                            ui_map,
                            t_manager,
                            ui.ui_scale.get(),
                        );
                    }
                }
                BrushAlfaObserve(alfa) => {
                    for id in callbacks.get(&BrushAlfaTxt).unwrap_or(&vec![]) {
                        Txt::update_text(
                            format!("{}", alfa),
                            *id,
                            ui_map,
                            t_manager,
                            ui.ui_scale.get(),
                        );
                    }
                }
                CanvasSlide => {
                    if (frame_count % ((1.0 / 60.0) / delta).ceil() as u32) == 0 {
                        let move_by = input
                            .pos
                            .substract(input.start_holding_at.unwrap())
                            .mult_one(-0.05);
                        c_manager.move_canvas(move_by);
                    }
                }
                _ => (),
            }
        }
    }
}
