# Coquelicot

## Introduction

Coquelicot is a x86_64 kernel trying to explore the possibilities of a linux
compatible kernel written in Rust exploring moderns ideas of kernel to make
the kernel easier to develop and maintain.

Coquelicot aims to provide a freestanding library (called `ostd`) that will
provide an safe interface to the kernel to interact with the hardware to improve
the security and the stability of the kernel. The kernel and its drivers will
be written in safe Rust, and only the `ostd` library will contain an
significant amount of unsafe code.

The `ostd` library will be also available as an ordinary Rust crate to allow
developers to easily write their own bare-metal applications in Rust with a
safe interface to the hardware and with less boilerplate code.

## Building

To build the kernel, you will need a nightly version of the Rust compiler,
installed preferably with `rustup`. Simply run `make run` at the root of the
repository to build the kernel and run it with QEMU. Common dependencies are
required to build the kernel, such as `lld`, `xorriso`, `qemu`...

## Contributing

Contributions are welcome! Feel free to open an issue or a pull request if you
want to help.

If you have a suggestion that would make this better, please fork the repo and
create a pull request. You can also simply open an issue with the tag
"enhancement". Don't forget to give the project a star! Thanks again!

## License

Coquelicot is dual-licensed under the Apache License, Version 2.0 and the MIT
license. See LICENSE-APACHE and LICENSE-MIT for details.
