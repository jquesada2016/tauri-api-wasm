#![feature(negative_impls)]

#[macro_use]
extern crate typed_builder;
#[macro_use]
extern crate serde;

#[macro_use]
mod macros;
pub mod global_shortcut;
pub mod tauri;
pub mod window;
