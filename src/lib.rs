pub mod audio;
pub mod colors;
pub mod input;
pub mod particles;
pub mod rapier;
pub mod winner;

pub mod prelude {
    pub use crate::audio::*;
    pub use crate::colors::*;
    pub use crate::input::*;
    pub use crate::particles::*;
    pub use crate::rapier::*;
    pub use crate::winner::*;
    pub use bevy::input::{
        common_conditions::input_toggle_active, gamepad::GamepadConnectionEvent,
    };
}
