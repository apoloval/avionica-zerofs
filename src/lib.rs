//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate libc;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate rustc_serialize;
extern crate toml;

mod config;
mod zmq;
mod zerofs;

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

#[cfg(windows)]
#[export_name="DLLStart"]
pub extern "stdcall" fn dll_start() {
    let settings = load_settings();
    init_logging(settings.logging);
    info!("Starting Avionica ZeroFS module");
    zerofs::start_module();
}

#[cfg(windows)]
#[export_name="DLLStop"]
pub extern "stdcall" fn dll_stop() {
    info!("Stopping Avionica ZeroFS module");
    zerofs::stop_module();
}
