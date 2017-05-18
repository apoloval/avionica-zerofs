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
    raw_context: *mut c_void
}

impl Context {
    pub fn new() -> Option<Context> {
        unsafe {
            from_raw_resource(
                ffi::zmq_ctx_new(),
                |raw| Context { raw_context: raw })
        }
    }

    pub fn socket(&self, socket_type: SocketType) -> Option<Socket> {
        unsafe {
            from_raw_resource(
                ffi::zmq_socket(self.raw_context, socket_type as c_int),
                |raw| Socket { _context: &self, _raw_socket: raw })
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum SocketType {
    PUB = 1,
    SUB = 2,
}

pub struct Socket<'a> {
    _context: &'a Context,
    _raw_socket: *mut c_void,
}

fn from_raw_resource<T, F: FnOnce(*mut c_void) -> T>(raw: *mut c_void, f: F) -> Option<T> {
    if (raw as isize) != -1 { Some(f(raw)) }
    else { None }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_context_creation() {
        assert!(Context::new().is_some());
    }

    #[test]
    fn test_socket_creation() {
        let ctx = Context::new().unwrap();
        assert!(ctx.socket(SocketType::PUB).is_some());
        assert!(ctx.socket(SocketType::SUB).is_some());
    }
}
