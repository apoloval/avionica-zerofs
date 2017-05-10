# Avionica ZeroFS

Avionica ZeroFS is a Flight Simulator/Prepar3D plugin to export the state of the simulation using [ZeroMQ][1].

This project is work in progress. Do not use it yet.

## How to build

Avionica ZeroFS is written in [Rust programming language][2]. The code is aimed to run on a 32 bits Windows environment.
You need to have Rust tools (using rustup is highly recommended) with the `stable-i686-pc-windows-msvc` toolchain
configured.

The code uses FFI bindings to libzmq C library. Thus, you have to indicate to Cargo (the Rust build system) where is
libzmq installed in your system. You can do that by:

* Declaring a `LIBZMQ_PREFIX` environment variable pointing to the ZMQ installation folder (e.g.,
`C:\Program Files (x86)\ZeroMQ 4.0.4`). Please note there must be a `zmq.lib` file in the `\lib` subfolder. If
necessary,
* Including the folder where ZMQ DLL resides. Make sure the `\bin` subfolder of ZMQ installation is included in your
`PATH` environment variable.

Once the environment is ready, you can build the project executing Cargo as in:

```
cargo build --release
```

[1]: http://zeromq.org/
[2]: https://www.rust-lang.org/
