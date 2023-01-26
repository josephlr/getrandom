// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for DragonFly BSD
use crate::{
    use_file,
    util::UninitBytes,
    util_libc::{sys_fill_exact, Weak},
    Error,
};
use core::{ffi::c_void, mem::MaybeUninit};

type GetRandomFn = unsafe extern "C" fn(*mut c_void, libc::size_t, libc::c_uint) -> libc::ssize_t;

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    // getrandom(2) was introduced in DragonflyBSD 5.7
    static GETRANDOM: Weak = unsafe { Weak::new("getrandom\0") };
    if let Some(fptr) = GETRANDOM.ptr() {
        let func: GetRandomFn = unsafe { core::mem::transmute(fptr) };
        return sys_fill_exact(dest, |buf| unsafe { func(buf.as_void_ptr(), buf.len(), 0) });
    }
    use_file::getrandom_inner(dest)
}
