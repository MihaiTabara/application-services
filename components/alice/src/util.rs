/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::error::*;
use nss_sys::*;

pub fn map_nss_secstatus<F>(callback: F) -> Result<()>
where
    F: FnOnce() -> SECStatus,
{
    if let SECSuccess = callback() {
        return Ok(());
    }
    let error_code = unsafe { nss_sys::PR_GetError() };
    Err(ErrorKind::NSSError(error_code).into())
}
