use super::{predefined::*, ui_element::*};

// #[derive(Clone, Copy)]
// /// unique name for each id_block
// pub enum BlockId {
//     MainLayout,
//     BrushSettings,
//     ForTest1,
// }
struct OverrideChildren {
    children: Vec<Vec<IdI32>>,
}

pub struct ElementMap;
impl ElementMap {
    pub fn init() -> Vec<UIElement> {
        let predefined = Predefined::new();
        // TODO: read custom layouts from file
        // let custom = IO::read_elements();
        // for i in 0..custom.len() {
        //     predefined[i].children = custom[i];
        // }
        predefined
    }
    pub fn init_layers() -> Vec<IdI32> {
        let mut layers = Vec::new();
        // TODO: read custom layouts from file
        // else
        layers.push(Id::RootMain as i32);
        layers
    }
}
