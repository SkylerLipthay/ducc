# ducc

I found that all of the other Duktape bindings were either outdated or
incomplete for my needs. Here's yet another interface, but with all of the
lessons I learned from using directly the unsafe FFI in a production setting.

## Inspiration

Deep gratitude to [kyren/rlua](https://github.com/kyren/rlua), which provided
inspiration and some directly copied code snippets. Lua and Duktape share a
very similar API, and so do `rlua` and `ducc`.

## To-do

* Serde integration (see `rlua_serde`).
* Storing user data (in `Ducc`) to be retrieved in Rust functions.
* Should all FFI code be wrapped in `protect`?
