//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::result;
use std::ffi::CString;

use libc::*;

mod ffi;

#[derive(Clone, Debug, PartialEq)]
pub struct Error(c_int);

pub type Result<T> = result::Result<T, Error>;

macro_rules! zmq_try {
    ($e:expr) => {{
        let rc = unsafe { $e };
        if (rc as isize) == -1 {
            return Err(Error(unsafe { ffi::zmq_errno() }));
        }
        rc
    }}
}

pub struct Context {
    raw_context: *mut c_void
}

impl Context {
    pub fn new() -> Result<Context> {
        let raw = zmq_try!(ffi::zmq_ctx_new());
        Ok(Context { raw_context: raw })
    }

    pub fn socket(&self, socket_type: SocketType) -> Result<Socket> {
        let raw = zmq_try!(ffi::zmq_socket(self.raw_context, socket_type as c_int));
        Ok(Socket { _context: &self, raw_socket: raw })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::zmq_ctx_destroy(self.raw_context); }
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
    raw_socket: *mut c_void,
}

impl<'a> Socket<'a> {
    pub fn bind<T: AsRef<str>>(&mut self, endpoint: T) -> Result<()> {
        let raw_endpoint = CString::new(endpoint.as_ref().as_bytes()).ok().unwrap();
        zmq_try!(ffi::zmq_bind(self.raw_socket, raw_endpoint.as_ptr()));
        Ok({})
    }

    pub fn connect<T: AsRef<str>>(&mut self, endpoint: T) -> Result<()> {
        let raw_endpoint = CString::new(endpoint.as_ref().as_bytes()).ok().unwrap();
        zmq_try!(ffi::zmq_connect(self.raw_socket, raw_endpoint.as_ptr()));
        Ok({})
    }
}

impl<'a> Drop for Socket<'a> {
    fn drop(&mut self) {
        unsafe { ffi::zmq_close(self.raw_socket); }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_context_creation() {
        assert!(Context::new().is_ok());
    }

    #[test]
    fn test_socket_creation() {
        let ctx = Context::new().ok().unwrap();
        assert!(ctx.socket(SocketType::PUB).is_ok());
        assert!(ctx.socket(SocketType::SUB).is_ok());
    }

    #[test]
    fn test_socket_connection() {
        let ctx = Context::new().ok().unwrap();
        let mut pub_socket = ctx.socket(SocketType::PUB).ok().unwrap();
        assert!(pub_socket.connect("tcp://localhost:5555").is_ok());
    }

    #[test]
    fn test_socket_binding() {
        let ctx = Context::new().ok().unwrap();
        let mut pub_socket = ctx.socket(SocketType::PUB).ok().unwrap();
        assert!(pub_socket.bind("tcp://*:5555").is_ok());
    }
}
