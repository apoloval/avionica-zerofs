//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use libc::*;

mod ffi;

pub struct Context {
    _raw_context: *mut c_void
}

impl Context {
    pub fn new() -> Option<Context> {
        unsafe {
            let raw = ffi::zmq_ctx_new();
            if (raw as isize) != -1 {
                Some(Context { _raw_context: raw })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_context_creation() {
        assert!(Context::new().is_some());
    }
}
