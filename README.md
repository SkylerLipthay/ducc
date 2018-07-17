# ducc

I found that all of the other Duktape bindings were either outdated or
incomplete for my needs. Here's yet another interface, but with all of the
lessons I learned from using directly the unsafe FFI in a production setting.

# To-do

* Finalize error management. Are the ergonomics right?
* Evaluate all `TODO` comments.
* Serde integration (see `rlua_serde`).
* Storing user data (in `Ducc`) to be retrieved in Rust functions.
  * Store a single `UserDataMap` pointer in the Duktape context, which is a
    struct containing a `BTreeMap<String, Rc<RefCell<Box<Any>>>>`
* Should all FFI code be wrapped in `protect`?
