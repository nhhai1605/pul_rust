use crate::my_libs::*;

pub fn get_window_size(window: Query<&Window>) -> (f32, f32) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    (width, height)
}