# Yttrium - Unity Web Runtime
Yttrium is an experimental, work-in-progress library for enabling
web features in Unity. The goal of the project is to integrate [Servo](https://servo.org),
the Mozilla web runtime, and [Deno](https://deno.land), a secure Javascript and Typescript
runtime environment.

## Overview
Yttrium is broken up into 2 parts, the native portion and the managed
portion. The native version is the runtime of the library;
it contains all necessary code to bootstrap the web renderer
and Deno runtime. The managed plugin is intended to ease
integration of the library into a Unity project.

## Yttrium Native

### Requirements
The only hard requirements are the Rust compiler, a C++ compiler,
and Unity, with the relevant version information found below.

| Requirement   | Version                        | Comments                                 |
|---------------|--------------------------------|------------------------------------------|
| Rust Compiler | 1.54.0-nightly                 | This version is tied directly to `servo` |
| C++ Compiler  | Any compiler supporting C++ 17 | For Windows, use MSVC 2019               |
| Unity         | 2021.3.12f1                    |                                          |

Take note that our rust version is tied **directly** to `servo`, as it is 
our primary dependency. We assume you are using `rustup`, and to
ensure the build succeeds, run the commands found below.

```bash
$ rustup component add rustc-dev
``` 

For Windows users, `vcpkg` is also a requirement. Detailed instructions on
installation can be found [here](https://vcpkg.io/en/getting-started.html),
but below is a concise guide.

```bash
$ git clone https://github.com/Microsoft/vcpkg.git
$ .\vcpkg\bootstrap-vcpkg.bat
$ .\vcpkg\vcpkg integrate install
```

After you've installed `vcpkg`, you need to install the `libopenssl`.
To do this, run the following commands.

```bash
$ .\vcpkg\vcpkg install openssl-windows:x64-windows
$ .\vcpkg\vcpkg install openssl:x64-windows-static
$ .\vcpkg\vcpkg install openssl:x64-windows-static-md
$ .\vcpkg\vcpkg integrate install
```

### Building
Building Yttrium is a fairly straightforward, and platform agnostic process.
The build process is outlined below.
```bash
$ mkdir build/ && cd build
$ cmake ..
$ make -j4
```

After building, add the library file (depending on platform it could be
either `libyttrium.so` or `yttrium.dll`) as a Unity dependency.

Note that when cross-compiling, you need to ensure you have the correct
version of the library for the operating system you're compiling for.
Most likely, your Unity development takes place on a Windows or Mac device,
so a cross-compilation environment for Linux will be required.

## Yttrium Unity

