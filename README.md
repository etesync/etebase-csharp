<p align="center">
  <img width="120" src="https://github.com/etesync/etesync-web/blob/master/src/images/logo.svg" />
  <h1 align="center">Etebase - Encrypt Everything</h1>
</p>

A C# library for Etebase

**Star** and **watch** for updates.

![GitHub tag](https://img.shields.io/github/tag/etesync/etebase-csharp.svg)
[![Build Status](https://github.com/etesync/etebase-csharp/workflows/Build/badge.svg)](https://github.com/etesync/etebase-csharp/actions/)
[![Chat with us](https://img.shields.io/badge/chat-IRC%20|%20Matrix%20|%20Web-blue.svg)](https://www.etebase.com/community-chat/)


# Documentation

The C# bindings are not yet documented, but the API is almost identical to the Python one, with the main difference being the CamelCase vs the snake case. See the docs at https://docs.etebase.com

# Using

We plan on adding a NuGet package soon (help needed!), though until then, you can download the libraries from the release page on github. The libraries are as follows:

* `etebase_csharp.dll` - the reference DLL that should be added to your project
* `libetebase_csharp_native.{so,dll,dylib}` - the per-platform Etebase library (actual Etebase code) - should be in the runtime path (e.g. on Windows it's just the same directory).

# Build

## Install deps

Make sure you have `cargo` (Rust Cargo) and `dotnet` (dotnet SDK) installed and working.

## Build

Build the main library and the C# p/invoke wrapper:
```
cargo build
cd target/dotnet-out/
dotnet build
```

You can then copy the libraries listed in the "Using" section above.
