//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod errno {
    use libc::c_int;

    // Use constants as defined in the windows header errno.h
    // libzmq should be compiled with VS2010 SDK headers or newer
    pub const EACCES: c_int = 13;
    pub const EADDRINUSE: c_int = 100;
    pub const EADDRNOTAVAIL: c_int = 101;
    pub const EAGAIN: c_int = 11;
    pub const EBUSY: c_int = 16;
    pub const ECONNREFUSED: c_int = 107;
    pub const EFAULT: c_int = 14;
    pub const EINTR: c_int = 4;
    pub const EHOSTUNREACH: c_int = 110;
    pub const EINPROGRESS: c_int = 112;
    pub const EINVAL: c_int = 22;
    pub const EMFILE: c_int = 24;
    pub const EMSGSIZE: c_int = 115;
    pub const ENAMETOOLONG: c_int = 38;
    pub const ENETDOWN: c_int = 116;
    pub const ENOBUFS: c_int = 119;
    pub const ENODEV: c_int = 19;
    pub const ENOENT: c_int = 2;
    pub const ENOMEM: c_int = 12;
    pub const ENOTCONN: c_int = 126;
    pub const ENOTSOCK: c_int = 128;
    pub const ENOTSUP: c_int = 129;
    pub const EPROTO: c_int = 134;
    pub const EPROTONOSUPPORT: c_int = 135;
}

use libc::*;

pub const ZMQ_DONTWAIT: c_int = 1;
pub const ZMQ_SNDMORE: c_int = 2;

#[link(name = "zmq")]
extern {
    pub fn zmq_bind(socket: *mut c_void, endpoint: *const c_char) -> c_int;
    pub fn zmq_close(socket: *mut c_void) -> c_int;
    pub fn zmq_connect(socket: *mut c_void, endpoint: *const c_char) -> c_int;
    pub fn zmq_ctx_destroy(context: *mut c_void) -> c_int;
    pub fn zmq_ctx_new() -> *mut c_void;
    pub fn zmq_errno() -> c_int;
    pub fn zmq_recv(socket: *mut c_void, buffer: *mut c_void, len: size_t, flags: c_int) -> c_int;
    pub fn zmq_send(socket: *mut c_void, buffer: *const c_void, len: size_t, flags: c_int) -> c_int;
    pub fn zmq_socket(context: *mut c_void, socket_type: c_int) -> *mut c_void;
}
