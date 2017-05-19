//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use libc::*;

pub const ZMQ_DONTWAIT: c_int = 1;
pub const ZMQ_SNDMORE: c_int = 2;

pub const ERRNO_EAGAIN: c_int = 11;

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
