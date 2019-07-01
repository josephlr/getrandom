// Copyright 2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::sync::atomic::{AtomicUsize, Ordering};

// TODO: Comment
#[derive(Debug)]
pub struct LazyUsize(AtomicUsize);

impl LazyUsize {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(usize::max_value()))
    }

    pub fn unsync_init(&self, init: impl FnOnce() -> usize) -> usize {
        if self.0.load(Ordering::Relaxed) == usize::max_value() {
            self.0.store(init(), Ordering::Relaxed)
        }
        self.0.load(Ordering::Relaxed)
    }
}

// TODO: Comment
pub struct LazyBool(LazyUsize);

impl LazyBool {
    pub const fn new() -> Self {
        Self(LazyUsize::new())
    }

    pub fn unsync_init(&self, init: impl FnOnce() -> bool) -> bool {
        self.0.unsync_init(|| init() as usize) != 0
    }
}
