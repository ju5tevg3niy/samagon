use std::any::type_name;
use std::ffi::CStr;
use std::ffi::c_char;
use std::fs;
use std::path::Path;

use anyhow::Context;
use serde::de::DeserializeOwned;

/// Get Rust String from C-string pointer
///
/// # Safety
/// See CStr
pub unsafe fn convert_cstr_to_string(cstr: *const c_char) -> String {
    unsafe { CStr::from_ptr(cstr) }
        .to_owned()
        .into_string()
        .expect("Failed to convert C-string to UTF-8 String")
}

pub fn read_json<P, T>(file: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file_data = fs::read_to_string(&file)
        .with_context(|| format!("Failed to read JSON file: {:?}", file.as_ref()))?;

    serde_json::from_str(&file_data)
        .with_context(|| format!("Failed to parse JSON file into {}", type_name::<T>()))
}
