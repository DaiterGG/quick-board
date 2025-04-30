use super::canvas_manager::StepId;

/// * `root_id`: Id of the root history step in the canvas manager history list
pub struct Layer {
    // maintain both, because
    // you need to draw from the root
    // but update the leaf when adding new leaf
    pub root_id: Option<StepId>,
    pub leaf_id: Option<StepId>,
}
impl Layer {
    pub fn new() -> Self {
        Self {
            root_id: None,
            leaf_id: None,
        }
    }
}
