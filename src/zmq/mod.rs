//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::CString;
use std::result;
use std::string::FromUtf8Error;

use libc::*;

mod ffi;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadMsg { bytes: Vec<u8> },
    BadOp(c_int),
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::BadMsg { bytes: error.into_bytes() }
    }
}

pub type Result<T> = result::Result<T, Error>;

macro_rules! zmq_try {
    ($e:expr) => {{
        let rc = unsafe { $e };
        if (rc as isize) == -1 {
            let error_code = unsafe { ffi::zmq_errno() };
            return Err(Error::BadOp(error_code));
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

    pub fn send<T: AsRef<[u8]>>(&mut self, data: T) -> Result<usize> {
        self.send_with_opts(data, false)
    }

    pub fn send_part<T: AsRef<[u8]>>(&mut self, data: T) -> Result<usize> {
        self.send_with_opts(data, true)
    }

    pub fn recv<T: AsMut<[u8]>>(&mut self, mut data: T) -> Result<bool> {
        let mut buffer: &mut [u8] = data.as_mut();
        let nbytes = buffer.len();
        let rc = unsafe {
            ffi::zmq_recv(
                self.raw_socket,
                buffer.as_ptr() as *mut c_void,
                nbytes,
                ffi::ZMQ_DONTWAIT)
        };
        if (rc as isize) == -1 {
            let error_code = unsafe { ffi::zmq_errno() };
            if error_code == ffi::ERRNO_EAGAIN { return Ok(false) }
            else { return Err(Error::BadOp(error_code)) }
        }
        Ok(true)
    }

    pub fn recv_string(&mut self) -> Result<Option<String>> {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        if self.recv(&mut buffer)? {
            Ok(Some(String::from_utf8(buffer)?))
        } else {
            Ok(None)
        }
    }

    fn send_with_opts<T: AsRef<[u8]>>(&mut self, data: T, send_more: bool) -> Result<usize> {
        let buffer: &[u8] = data.as_ref();
        let nbytes = buffer.len();
        let flags = if send_more { ffi::ZMQ_SNDMORE } else { 1 };
        let nbytes = zmq_try!(ffi::zmq_send(
            self.raw_socket,
            buffer.as_ptr() as *const c_void,
            nbytes,
            flags));
        Ok(nbytes as usize)
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

    #[test]
    fn test_socket_send() {
        let ctx = Context::new().ok().unwrap();
        let mut pub_socket = ctx.socket(SocketType::PUB).ok().unwrap();
        assert!(pub_socket.bind("tcp://*:5556").is_ok());
        assert_eq!(Ok(6), pub_socket.send_part("foobar"));
        assert_eq!(Ok(12), pub_socket.send("Hello World!"));
    }

    #[test]
    fn test_socket_recv() {
        let ctx = Context::new().ok().unwrap();
        let mut sub_socket = ctx.socket(SocketType::SUB).ok().unwrap();
        assert!(sub_socket.bind("tcp://*:5557").is_ok());
        assert!(sub_socket.recv_string().is_ok());
    }
}
