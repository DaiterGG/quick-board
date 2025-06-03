use super::brush_tool::Brush;
use super::canvas_manager::CanvasData;
use super::fill_tool::Fill;
use super::move_tool::Move;
use super::sample_tool::Sample;
use super::texture_manager::TextureManager;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ToolId {
    Fill,
    Sample,
    Brush,
    Move,
}
pub struct Tools {
    pub fill: Fill,
    pub sample: Sample,
    pub brush: Brush,
    pub move_tool: Move,
}
impl Tools {
    pub fn init_all_tools(t_manager: &mut TextureManager) -> Self {
        Self {
            fill: Fill::new(),
            sample: Sample::new(),
            brush: Brush::new(t_manager),
            move_tool: Move::new(),
        }
    }

    // pub fn enable(&self, tool_id: ToolId, data: &mut CanvasData) {
    //     match tool_id {
    //         ToolId::Brush => self.brush.enable(data),
    //         // ToolId::Move => self.move_tool.enable(data),
    //         // ToolId::Fill => self.fill.enable(data),
    //         _ => {}
    //     }
    // }
    pub fn get_size(&self, tool_id: ToolId) -> i32 {
        match tool_id {
            ToolId::Brush => self.brush.brush_diameter(),
            _ => -1,
        }
    }
    pub fn add_size(&mut self, tool_id: ToolId, add: f32, t_manager: &mut TextureManager) {
        match tool_id {
            ToolId::Brush => self.brush.add_size(t_manager, add),
            _ => {}
        }
    }
    pub fn mult_size(&mut self, tool_id: ToolId, by: f32, t_manager: &mut TextureManager) {
        match tool_id {
            ToolId::Brush => self.brush.mult_size(t_manager, by),
            _ => {}
        }
    }
}
