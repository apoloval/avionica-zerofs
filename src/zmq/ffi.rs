//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use libc::*;

#[link(name = "zmq")]
extern {
    pub fn zmq_ctx_new() -> *mut c_void;
    pub fn zmq_socket(context: *mut c_void, socket_type: c_int) -> *mut c_void;
}
