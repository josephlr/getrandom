// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(non_camel_case_types, clippy::upper_case_acronyms)]
use crate::{util::LazyBool, Error};
use core::{convert::TryInto, ffi::c_void, mem::MaybeUninit, num::NonZeroU32, ptr};

type NTSTATUS = u32;
type BCRYPT_ALG_HANDLE = *mut c_void;

const STATUS_INVALID_HANDLE: NTSTATUS = 0xC0000008;
// TODO: Replace with ptr::invalid_mut(0x00000081) when that is stable.
const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000081 as BCRYPT_ALG_HANDLE;
const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;

#[link(name = "bcrypt")]
extern "system" {
    fn BCryptGenRandom(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        pBuffer: *mut u8,
        cbBuffer: u32,
        dwFlags: u32,
    ) -> NTSTATUS;
}

fn has_rng_alg_handle() -> bool {
    let ret = unsafe { BCryptGenRandom(BCRYPT_RNG_ALG_HANDLE, ptr::null_mut(), 0, 0) };
    ret != STATUS_INVALID_HANDLE
}

// BCryptGenRandom was introduced in Windows Vista.
fn bcrypt_random(s: &mut [MaybeUninit<u8>]) -> NTSTATUS {
    let ptr = s.as_mut_ptr() as *mut u8;
    // Will always succeed given the chunking below.
    let len: u32 = s.len().try_into().unwrap();

    static HAS_RNG_ALG_HANDLE: LazyBool = LazyBool::new();
    if HAS_RNG_ALG_HANDLE.unsync_init(has_rng_alg_handle) {
        unsafe { BCryptGenRandom(BCRYPT_RNG_ALG_HANDLE, ptr, len, 0) }
    } else {
        unsafe { BCryptGenRandom(ptr::null_mut(), ptr, len, BCRYPT_USE_SYSTEM_PREFERRED_RNG) }
    }
}

fn status_to_result(ret: NTSTATUS) -> Result<(), Error> {
    // NTSTATUS codes use the two highest bits for severity status.
    if ret >> 30 != 0b11 {
        return Ok(());
    }
    // We zeroize the highest bit, so the error code will reside
    // inside the range designated for OS codes.
    let code = ret ^ (1 << 31);
    // SAFETY: the second highest bit is always equal to one,
    // so it's impossible to get zero. Unfortunately the type
    // system does not have a way to express this yet.
    let code = unsafe { NonZeroU32::new_unchecked(code) };
    Err(Error::from(code))
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    // Prevent overflow of u32
    for chunk in dest.chunks_mut(u32::max_value() as usize) {
        status_to_result(bcrypt_random(chunk))?;
    }
    Ok(())
}
