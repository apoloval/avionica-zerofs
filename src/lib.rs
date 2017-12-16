//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate byteorder;
extern crate libc;
extern crate log4rs;
extern crate nanomsg;
extern crate serde;
extern crate toml;

mod config;
mod channel;
mod event;

const CONFIG_FILE: &'static str = "Modules/zerofs.conf";

fn load_settings() -> config::Settings {
    config::Settings::from_toml_file(CONFIG_FILE)
        .ok()
        .unwrap_or_else(|| {
            println!("Avionica ZeroFS cannot load config file at {}", CONFIG_FILE);
            println!("Falling back to default settings");
            config::Settings::default()
        })
}

fn init_logging(settings: config::LoggingSettings) {
    log4rs::init_config(log4rs::config::Config::from(settings)).unwrap();
}

macro_rules! init {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                error!("Initialization error: {}", e);
                return
            },
        }
    };
}

#[cfg(windows)]
#[export_name="DLLStart"]
pub extern "stdcall" fn dll_start() {
    let settings = load_settings();
    init_logging(settings.logging);
    info!("Starting Avionica ZeroFS module");

    let _pub_chan = init!(channel::nano::PubChannel::bind("tcp://localhost:9000"));
}

#[cfg(windows)]
#[export_name="DLLStop"]
pub extern "stdcall" fn dll_stop() {
    info!("Stopping Avionica ZeroFS module");
}
