use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Jump,
    Run,
}

impl Actionlike for Action {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Action::Jump => InputControlKind::Button,
            Action::Run => InputControlKind::Axis,
        }
    }
}
