/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::types::nspr::*;
use crate::types::nss3::*;
use std::os::raw::{c_char, c_uchar};
use std::sync::atomic::{AtomicBool, Ordering};

static NSS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(crate) fn ensure_initialized() {
    unsafe {
        let NSS_IsInitialized_HANDLE: libloading::Symbol<unsafe extern "C" fn() -> PRBool> =
            LIBNSS3
                .get(b"NSS_IsInitialized")
                .expect("Could not get NSS_IsInitialized handle.");
        if !NSS_INITIALIZED.swap(true, Ordering::Relaxed) && NSS_IsInitialized_HANDLE() == PR_FALSE
        {
            let NSS_InitContext: libloading::Symbol<
                unsafe extern "C" fn(
                    configdir: *const c_char,
                    certPrefix: *const c_char,
                    keyPrefix: *const c_char,
                    secmodName: *const c_char,
                    initParams: *mut NSSInitParameters,
                    flags: PRUint32,
                ) -> *mut NSSInitContext,
            > = LIBNSS3
                .get(b"NSS_InitContext")
                .expect("Could not get NSS_InitContext handle.");
            // We use the same args as NSS_NoDB_Init.
            let empty = std::ffi::CString::default();
            let flags = NSS_INIT_READONLY
                | NSS_INIT_NOCERTDB
                | NSS_INIT_NOMODDB
                | NSS_INIT_FORCEOPEN
                | NSS_INIT_OPTIMIZESPACE;
            NSS_InitContext(
                empty.as_ptr(),
                empty.as_ptr(),
                empty.as_ptr(),
                empty.as_ptr(),
                std::ptr::null_mut(),
                flags,
            );
        }
    }
}

lazy_static::lazy_static! {
    // Lib handle.
    pub(crate) static ref LIBNSS3: libloading::Library = {
        libloading::Library::new("libnss3.so").expect("libnss3.so should be already loaded.")
    };
    // Function handles.
    pub(crate) static ref NSS_IsInitialized: libloading::Symbol<'static, unsafe extern fn() -> PRBool> = {
        unsafe {
            LIBNSS3.get(b"NSS_IsInitialized").expect("Could not get NSS_IsInitialized handle.")
        }
    };
    pub(crate) static ref NSS_GetVersion: libloading::Symbol<'static, unsafe extern fn() -> *const c_char> = {
        unsafe {
            LIBNSS3.get(b"NSS_GetVersion").expect("Could not get NSS_GetVersion handle.")
        }
    };
    pub(crate) static ref PR_GetError: libloading::Symbol<'static, unsafe extern fn() -> PRErrorCode> = {
        unsafe {
            LIBNSS3.get(b"PR_GetError").expect("Could not get PR_GetError handle.")
        }
    };
    pub(crate) static ref PK11_HashBuf: libloading::Symbol<'static, unsafe extern fn(hashAlg: SECOidTag, out: *mut c_uchar, r#in: *const c_uchar, len: PRInt32) -> SECStatus> = {
        unsafe {
            LIBNSS3.get(b"PK11_HashBuf").expect("Could not get PK11_HashBuf handle.")
        }
    };
}
