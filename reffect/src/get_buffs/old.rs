//! A manual implementation of loading the DLL at runtime.
//! Replaced by Rusts own linking system (for now).

use libloading::os::windows::{Library, Symbol};
use std::cell::OnceCell;
use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf, slice};
use windows::Win32::{Foundation::MAX_PATH, System::LibraryLoader::GetModuleFileNameW};

static MY_HANDLE: OnceLock<HMODULE> = OnceLock::new();

#[no_mangle]
extern "system" fn DllMain(handle: HMODULE, reason: u32, _: *mut ()) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => {
            let _ = MY_HANDLE.set(handle);
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    true
}

type GetCurrentPlayerStackedBuffs = extern "C" fn() -> *const StackedBuff;

static mut GET_BUFFS: OnceCell<GetBuffs> = OnceCell::new();

#[derive(Debug)]
pub struct GetBuffs {
    _library: Library,
    get_current_player_buffs: Symbol<GetCurrentPlayerStackedBuffs>,
}

impl GetBuffs {
    fn new(
        library: Library,
        get_current_player_buffs: Symbol<GetCurrentPlayerStackedBuffs>,
    ) -> Self {
        Self {
            _library: library,
            get_current_player_buffs,
        }
    }

    pub fn load() {
        // should not happen while accessing from multiple threads
        let _ = unsafe { &GET_BUFFS }.set(unsafe { GetBuffs::load_library() });
    }

    pub fn unload() {
        // should not happen while accessing from multiple threads
        unsafe { &mut GET_BUFFS }.take();
    }

    pub unsafe fn get_buffs<'a>() -> Result<&'a [StackedBuff], GetBuffsError> {
        GET_BUFFS
            .get()
            .expect("getbuffs.dll not loaded")
            .call_export()
    }

    unsafe fn load_library() -> Self {
        let mut buffer = [0; MAX_PATH as usize];
        let len = GetModuleFileNameW(
            *MY_HANDLE.get().expect("own handle not initialized"),
            &mut buffer,
        );
        if len == 0 {
            panic!("no module filename for current module")
        }
        let module_path = PathBuf::from(OsString::from_wide(&buffer));
        let lib_path = module_path.with_file_name("getbuffs.dll");
        let lib = Library::new(lib_path).expect("error loading getbuffs.dll");
        let func = lib
            .get(b"GetCurrentPlayerStackedBuffs\0")
            .expect("error retrieving GetCurrentPlayerStackedBuffs");

        Self::new(lib, func)
    }

    unsafe fn call_export<'a>(&self) -> Result<&'a [StackedBuff], GetBuffsError> {
        let buffs = (*self.get_current_player_buffs)();
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
}
