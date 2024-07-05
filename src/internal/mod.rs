mod shared;

pub use self::shared::*;

use std::slice;

// link as raw dynamic library (windows only)
// see https://doc.rust-lang.org/reference/items/external-blocks.html#dylib-versus-raw-dylib
// FIXME: required to be next to exe! manual load instead?
#[link(name = "reffect_internal", kind = "raw-dylib")]
extern "C" {
    pub fn initialize();

    fn update_buffs() -> BuffsResult;
}

/// Returns the buffs currently present on the player or an error.
///
/// # Safety
/// This is unsafe due to the caller choosing the lifetime of the buff slice.
pub unsafe fn get_buffs<'a>() -> Option<&'a [Buff]> {
    let result = unsafe { update_buffs() };
    (!result.error).then(|| unsafe { slice::from_raw_parts(result.buffs, result.len) })
}
