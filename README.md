# ducc

I found that all of the other Duktape bindings were either outdated or
incomplete for my needs. Here's yet another interface, but with all of the
lessons I learned from using directly the unsafe FFI in a production setting.

## Inspiration

Deep gratitude to [kyren/rlua](https://github.com/kyren/rlua), which provided
inspiration and some directly copied code snippets. Lua and Duktape share a
very similar API, and so do `rlua` and `ducc`. Also incredibly inspirational was
[zrkn/rlua_serde](https://github.com/zrkn/rlua_serde).
