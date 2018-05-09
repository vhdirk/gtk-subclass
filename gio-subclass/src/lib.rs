
#[macro_use]
extern crate glib;
extern crate gio;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;

#[macro_use]
extern crate gobject_subclass;

// If we make application private (which it should be), `box_gapplication_impl` stops working.
pub mod application;
pub use application::*;
