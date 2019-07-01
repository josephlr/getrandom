// Copyright 2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::lazy::LazyUsize;
use core::marker::PhantomData;
use core::mem;

// TODO: Comment
pub struct Weak<F> {
    name: &'static str,
    addr: LazyUsize,
    _marker: PhantomData<F>,
}

impl<F> Weak<F> {
    // Safety, name must match F, may panic if F is not a function pointer.
    pub const unsafe fn new(name: &'static str) -> Self {
        Self {
            name: name,
            addr: LazyUsize::new(),
            _marker: PhantomData,
        }
    }

    pub fn func(&self) -> Option<F> {
        assert_eq!(mem::size_of::<Option<F>>(), mem::size_of::<usize>());

        let addr = self.addr.unsync_init(|| unsafe {
            libc::dlsym(libc::RTLD_DEFAULT, self.name.as_ptr() as *const _) as usize
        });
        unsafe { mem::transmute_copy(&addr) }
    }
}
