/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(not(feature = "use_geckoview_nss"))]
mod dylib_nss;
#[cfg(feature = "use_geckoview_nss")]
mod geckoview_nss;

#[cfg(not(feature = "use_geckoview_nss"))]
pub(crate) use dylib_nss::*;
#[cfg(feature = "use_geckoview_nss")]
pub(crate) use geckoview_nss::*;
