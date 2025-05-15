use super::brush_tool::Brush;
use super::fill_tool::Fill;
use super::move_tool::Move;
use super::texture_manager::TextureManager;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ToolId {
    Fill,
    Brush,
    Move,
}
pub struct Tools {
    pub fill: Fill,
    pub brush: Brush,
    pub move_tool: Move,
}
impl Tools {
    pub fn init_all_tools(t_manager: &mut TextureManager) -> Self {
        Self {
            fill: Fill::new(),
            brush: Brush::new(t_manager),
            move_tool: Move::new(),
        }
    }
}
