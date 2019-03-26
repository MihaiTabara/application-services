/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

fn env(name: &str) -> Option<OsString> {
    println!("cargo:rerun-if-env-changed={}", name);
    env::var_os(name)
}

fn main() {
    let (lib_dir, include_dir) = get_nss();
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_string_lossy()
    );
    println!("cargo:include={}", include_dir.to_string_lossy());
}

// Adapted from https://github.com/sfackler/rust-openssl/blob/546405dc58e0e11d695ba6957aab567f9d559192/openssl-sys/build/find_normal.rs

pub fn get_nss() -> (PathBuf, PathBuf) {
    let nss_dir = env("NSS_DIR").unwrap_or_else(|| find_nss_dir());
    let nss_dir = Path::new(&nss_dir);
    let lib_dir = nss_dir.join("lib");
    let include_dir = nss_dir.join("include");
    (lib_dir, include_dir)
}

fn find_nss_dir() -> OsString {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    if host == target && target.contains("apple-darwin") {
        // Check up default Homebrew installation location first
        // for quick resolution if possible.
        let homebrew = Path::new("/usr/local/opt/nss");
        if homebrew.exists() {
            return homebrew.to_path_buf().into();
        }
        // Calling `brew --prefix <package>` command usually slow and
        // takes seconds, and will be used only as a last resort.
        let output = execute_command_and_get_output("brew", &["--prefix", "nss"]);
        if let Some(ref output) = output {
            let homebrew = Path::new(&output);
            if homebrew.exists() {
                return homebrew.to_path_buf().into();
            }
        }
    }

    try_pkg_config();
    try_vcpkg();

    panic!("Could not find NSS!");
}

/// Attempt to find NSS through pkg-config.
///
/// Note that if this succeeds then the function does not return as pkg-config
/// typically tells us all the information that we need.
fn try_pkg_config() {
    // let target = env::var("TARGET").unwrap();
    // let host = env::var("HOST").unwrap();

    // // If we're going to windows-gnu we can use pkg-config, but only so long as
    // // we're coming from a windows host.
    // //
    // // Otherwise if we're going to windows we probably can't use pkg-config.
    // if target.contains("windows-gnu") && host.contains("windows") {
    //     env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
    // } else if target.contains("windows") {
    //     return;
    // }

    // let lib = match pkg_config::Config::new()
    //     .print_system_libs(false)
    //     .find("nss3")
    // {
    //     Ok(lib) => lib,
    //     Err(e) => {
    //         println!("run pkg_config fail: {:?}", e);
    //         return;
    //     }
    // };

    // for include in lib.include_paths.iter() {
    //     println!("cargo:include={}", include.display());
    // }

    // process::exit(0);
}

/// Attempt to find NSS through vcpkg.
///
/// Note that if this succeeds then the function does not return as vcpkg
/// should emit all of the cargo metadata that we need.
#[cfg(target_env = "msvc")]
fn try_vcpkg() {
    // // vcpkg will not emit any metadata if it can not find libraries
    // // appropriate for the target triple with the desired linkage.
    // let mut lib = vcpkg::Config::new()
    //     .emit_includes(true)
    //     .lib_name("libnss3")
    //     .probe("nss");

    // let lib = lib.unwrap();
    // println!("cargo:rustc-link-lib=nss3");
    // process::exit(0);
}

#[cfg(not(target_env = "msvc"))]
fn try_vcpkg() {}

fn execute_command_and_get_output(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output();
    if let Ok(ref r1) = out {
        if r1.status.success() {
            let r2 = String::from_utf8(r1.stdout.clone());
            if let Ok(r3) = r2 {
                return Some(r3.trim().to_string());
            }
        }
    }
    return None;
}
