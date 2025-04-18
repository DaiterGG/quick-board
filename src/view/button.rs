use super::{
    action_state::Action, pointer_state::ButtonState, states::States, ui_builder::Id,
    ui_element::UIElement,
};

pub struct Button {}
// impl Button {
//     pub const fn new(id: Id, bg_color: Option<Color>) -> Div {
//     }
// }
impl Button {
    pub fn before_collision(element: &UIElement, states: &mut States) {
        let pntr = &mut states.pointer;
        if pntr.left == ButtonState::Pressed {
            // NOTE: this will only register the last sub button
            // TEST: this interaciton later
            pntr.interacting_with = Some(element.id);
            println!("interacting ");
        }
        if pntr.left == ButtonState::Released && pntr.interacting_with == Some(element.id) {
            states.action.add(Action::ButtonPressed(element.id));
            println!("action registered")
        }
    }
    pub fn after_collision(element: &UIElement, states: &mut States) {
        let pntr = &mut states.pointer;
    }
}
