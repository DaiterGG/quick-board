use super::brush_tool::Brush;
use super::fill_tool::Fill;
use super::move_tool::Move;

macro_rules! tools{
    (
        $(
            $field:ident : $tool:ident
        ),* $(,)?
    ) => {
        #[derive(Copy, Clone,Eq, PartialEq)]
        pub enum ToolId{
            $(
                $tool,
            )*
        }
        pub struct Tools {
            $(
                pub $field: $tool,
            )*
        }
        impl Tools {
            pub fn init_all_tools() -> Self{
                Self {
                    $(
                        $field: $tool::new(),
                    )*
                }
            }
        }
    };
}

tools! {
    fill: Fill,
    brush: Brush,
    move_tool: Move,
}
