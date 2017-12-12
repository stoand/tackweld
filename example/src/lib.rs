#![feature(wasm_import_memory)]
#![wasm_import_memory]
#[macro_use]
extern crate tackweld;

use std::ffi::CString;
use std::os::raw::c_char;

// share memory with javascript
pub mod memory;

fn get_dom() -> String {
    use tackweld::*;

    let vals = [1, 2, 3, 3, 4];

    let items = vals.iter()
        .map(|v| tw!(item, val = val(v)))
        .collect::<Vec<_>>();

    tw!(root, items = items).to_string()
    // "asdfasdf".into()
}

#[no_mangle]
pub extern "C" fn roundtrip(data: *mut c_char) -> *mut c_char {
    let mut a = unsafe { CString::from_raw(data).into_string().unwrap() };

    a = get_dom();

    CString::new(a.as_bytes()).unwrap().into_raw()
}
