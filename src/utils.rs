use std::ffi::CStr;
use std::ffi::c_char;

pub unsafe fn convert_cstr_to_string(cstr: *const c_char) -> String {
    unsafe { CStr::from_ptr(cstr) }
        .to_owned()
        .into_string()
        .expect("Failed to convert C-string to UTF-8 String")
}
