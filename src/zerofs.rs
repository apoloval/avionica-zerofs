//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::mem::size_of;
use std::ptr;

use libc::malloc;

use zmq;

pub struct Module {
    _zmq_ctx: zmq::Context
}

impl Module {
    pub fn new() -> Module {
        Module {
            _zmq_ctx: zmq::Context::new().unwrap()
        }
    }

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}
}

static mut MODULE: *mut Module = 0 as *mut Module;

pub fn start_module() {
    unsafe {
        MODULE = malloc(size_of::<Module>()) as *mut Module;
        ptr::write(MODULE, Module::new());
        (*MODULE).start();
    }
}

pub fn stop_module() {
    unsafe {
        let mut m = ptr::read(MODULE);
        m.stop();
    }
}
