mod camera;
mod constants;
mod fps;
mod table;
mod utils;

pub mod my_libs {
    pub use bevy::{diagnostic::*, prelude::*, sprite::*};
    pub use crate::camera::*;
    pub use crate::fps::*;
    pub use crate::table::*;
    pub use crate::utils::*;
    pub use crate::constants::*;
}
