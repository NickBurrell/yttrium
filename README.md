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
and Unity, with the relevant versioning information found below.

| Requirement   | Version                        | Comments                   |
|---------------|--------------------------------|----------------------------|
| Rust Compiler | 1.53.0                         |                            |
| C++ Compiler  | Any compiler supporting C++ 17 | For Windows, use MSVC 2019 |
| Unity         | 2021.3.12f1                    |                            |

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

