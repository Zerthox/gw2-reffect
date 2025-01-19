use nexus::imgui::sys;
use std::ptr;

pub fn next_window_size_constraints(size_min: [f32; 2], size_max: [f32; 2]) {
    unsafe {
        sys::igSetNextWindowSizeConstraints(size_min.into(), size_max.into(), None, ptr::null_mut())
    }
}
