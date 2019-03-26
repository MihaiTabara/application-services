/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

mod backend;
mod types;

use std::os::raw::{c_char, c_uchar};
pub use types::nspr::*;
pub use types::nss3::*;

pub unsafe fn PK11_HashBuf(
    hashAlg: SECOidTag,
    out: *mut c_uchar,
    r#in: *const c_uchar,
    len: PRInt32,
) -> SECStatus {
    backend::ensure_initialized();
    backend::PK11_HashBuf(hashAlg, out, r#in, len)
}

pub unsafe fn NSS_IsInitialized() -> PRBool {
    backend::ensure_initialized();
    backend::NSS_IsInitialized()
}

pub unsafe fn NSS_GetVersion() -> *const c_char {
    backend::ensure_initialized();
    backend::NSS_GetVersion()
}

pub unsafe fn PR_GetError() -> PRErrorCode {
    backend::ensure_initialized();
    backend::PR_GetError()
}
