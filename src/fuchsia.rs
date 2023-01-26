// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for Fuchsia Zircon
use crate::{util::UninitBytes, Error};
use core::{ffi::c_void, mem::MaybeUninit};

#[link(name = "zircon")]
extern "C" {
    fn zx_cprng_draw(buffer: *mut c_void, length: usize);
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    unsafe { zx_cprng_draw(dest.as_void_ptr(), dest.len()) }
    Ok(())
}
