# PawKit

PawKit is a mninimal game engine toolkit, written in Rust, with support for C, C++, and Lua.

In the future PawKit will support C#, Godot, and Javascript.

## Why a monorepo?

A monorepo helps me organize better, and keep everything working with the latest versions of each other.

## Notes about Lua bindings

Since Lua doesn't support multithreaded execution, Lua is unable to define logger callbacks, as those callbacks are intended to be accessed from multiple threads.
