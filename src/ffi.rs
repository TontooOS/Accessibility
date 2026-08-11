//! C FFI exports for Accessibility library
//!
//! This module exposes the i18n functionality via C-compatible symbols
//! so that non-Rust programs (C, C++, Python via ctypes, etc.) can use
//! the Accessibility framework.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

static LANG_MUTEX: Mutex<()> = Mutex::new(());

fn with_lang<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = LANG_MUTEX.lock().unwrap();
    f()
}

/// Initialize the library with a fallback language.
///
/// # Safety
/// `fallback` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn accessibility_init(fallback: *const c_char) -> i32 {
    let _guard = LANG_MUTEX.lock().unwrap();

    if fallback.is_null() {
        return -1;
    }

    let fallback_str = match CStr::from_ptr(fallback).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };

    let result = with_lang(|| super::init_lang(&fallback_str));

    match result {
        Ok(()) => 0,
        Err(_) => -3,
    }
}

/// Translate a key using the specified language.
///
/// # Safety
/// `lang` and `key` must be valid null-terminated C strings.
/// The returned pointer must be freed with `accessibility_free_string`.
#[no_mangle]
pub unsafe extern "C" fn accessibility_translate(
    lang: *const c_char,
    key: *const c_char,
) -> *mut c_char {
    if lang.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }

    let lang_str = match CStr::from_ptr(lang).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let key_str = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = with_lang(|| {
        let store = crate::LangStore::instance();
        store
            .t(lang_str, key_str, None)
            .unwrap_or_else(|| key_str.to_string())
    });

    CString::new(result).unwrap_or_default().into_raw()
}

/// Get the library version string.
///
/// The returned pointer is static and must NOT be freed.
#[no_mangle]
pub extern "C" fn accessibility_version() -> *const c_char {
    concat!("0.1.0", "\0").as_ptr() as *const c_char
}

/// Free a string previously returned by `accessibility_translate`.
///
/// # Safety
/// `ptr` must have been returned by `accessibility_translate` and not already freed.
#[no_mangle]
pub unsafe extern "C" fn accessibility_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

/// Get the number of loaded languages.
#[no_mangle]
pub extern "C" fn accessibility_lang_count() -> i32 {
    let _guard = LANG_MUTEX.lock().unwrap();
    let store = crate::LangStore::instance();
    store.all_langs().len() as i32
}

/// Get the language code at the given index.
///
/// The returned pointer is static and must NOT be freed.
/// Returns null if index is out of bounds.
#[no_mangle]
pub unsafe extern "C" fn accessibility_lang_at(index: i32) -> *const c_char {
    let _guard = LANG_MUTEX.lock().unwrap();
    let store = crate::LangStore::instance();
    let langs = store.all_langs();
    if index < 0 || index >= langs.len() as i32 {
        return std::ptr::null();
    }
    CString::new(langs[index as usize])
        .unwrap_or_default()
        .into_raw() as *const c_char
}
