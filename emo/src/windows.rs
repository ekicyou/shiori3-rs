#![cfg(any(windows))]
use shiori3::api::RawAPI;
use std::default::Default;
use std::ptr;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

lazy_static! {
    static ref api: RawAPI<super::shiori::EmoShiori> = Default::default();
}

#[no_mangle]
pub extern "C" fn load(h_dir: HGLOBAL, len: usize) -> bool {
    (*api).raw_shiori3_load(h_dir, len)
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    (*api).raw_shiori3_unload()
}

#[no_mangle]
pub extern "C" fn request(h: HGLOBAL, len: &mut usize) -> HGLOBAL {
    (*api).raw_shiori3_request(h, len)
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    h_inst: usize,
    ul_reason_for_call: DWORD,
    lp_reserved: LPVOID,
) -> bool {
    (*api).raw_shiori3_dll_main(h_inst, ul_reason_for_call, lp_reserved)
}
