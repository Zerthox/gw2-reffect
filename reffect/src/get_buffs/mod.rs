mod buff;
mod error;

pub use self::buff::*;
pub use self::error::*;

use std::slice;

// link as raw dynamic library (windows only)
// see https://doc.rust-lang.org/reference/items/external-blocks.html#dylib-versus-raw-dylib
#[link(name = "getbuffs", kind = "raw-dylib")]
extern "C" {
    fn GetCurrentPlayerStackedBuffs() -> *const StackedBuff;
}

/// Returns the buffs currently present on the player or an error.
///
/// # Safety
/// This is unsafe due to the caller choosing the lifetime of the buff slice.
pub unsafe fn get_buffs<'a>() -> Result<&'a [StackedBuff], GetBuffsError> {
    let buffs = GetCurrentPlayerStackedBuffs();
    if !buffs.is_null() {
        let first = &*buffs;
        match first.error() {
            Some(err) => Result::Err(err.into()),
            None => {
                let mut count = 1;
                while let Some(false) = buffs.add(count).as_ref().map(|buff| buff.is_end()) {
                    count += 1;
                }
                Result::Ok(slice::from_raw_parts(buffs, count))
            }
        }
    } else {
        Result::Err(GetBuffsError::Null)
    }
}
