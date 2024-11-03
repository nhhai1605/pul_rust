mod main_camera;
mod constants;
mod hud;
mod table;
mod utils;
pub mod my_libs {
    pub use crate::main_camera::*; // Replace OtherNeededItem with actual items needed
    pub use crate::constants::*;
    pub use crate::hud::*;
    pub use crate::table::*;
    pub use crate::utils::*;
    pub use bevy::{
        diagnostic::*,
        prelude::*,
        render::{camera::*, render_resource::*, view::*},
        sprite::*,
        window::*,
    };
}
