// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for iOS
use crate::{util::UninitBytes, Error};
use core::{ffi::c_void, mem::MaybeUninit, ptr::null};

#[link(name = "Security", kind = "framework")]
extern "C" {
    fn SecRandomCopyBytes(rnd: *const c_void, count: usize, bytes: *mut c_void) -> i32;
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    // Apple's documentation guarantees kSecRandomDefault is a synonym for NULL.
    let ret = unsafe { SecRandomCopyBytes(null(), dest.len(), dest.as_void_ptr()) };
    // errSecSuccess (from SecBase.h) is always zero.
    if ret != 0 {
        Err(Error::IOS_SEC_RANDOM)
    } else {
        Ok(())
    }
}
