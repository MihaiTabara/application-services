/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::types::nspr::*;
use crate::types::nss3::*;
use std::os::raw::{c_char, c_uchar};
use std::sync::atomic::{AtomicBool, Ordering};

static NSS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(crate) fn ensure_initialized() {
    if !NSS_INITIALIZED.swap(true, Ordering::Relaxed) && unsafe { NSS_IsInitialized() } == PR_FALSE
    {
        let empty = std::ffi::CString::default();
        let flags = NSS_INIT_READONLY
            | NSS_INIT_NOCERTDB
            | NSS_INIT_NOMODDB
            | NSS_INIT_FORCEOPEN
            | NSS_INIT_OPTIMIZESPACE;
        unsafe {
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

#[link(name = "nss3")]
#[link(name = "nspr4")]
extern "C" {
    pub(crate) fn PK11_HashBuf(
        hashAlg: SECOidTag,
        out: *mut c_uchar,
        r#in: *const c_uchar,
        len: PRInt32,
    ) -> SECStatus;
    pub(crate) fn NSS_InitContext(
        configdir: *const c_char,
        certPrefix: *const c_char,
        keyPrefix: *const c_char,
        secmodName: *const c_char,
        initParams: *mut NSSInitParameters,
        flags: PRUint32,
    ) -> *mut NSSInitContext;
    pub(crate) fn NSS_IsInitialized() -> PRBool;
    pub(crate) fn NSS_GetVersion() -> *const c_char;
    pub(crate) fn PR_GetError() -> PRErrorCode;
}
