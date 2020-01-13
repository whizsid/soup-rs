
#![allow(unused_imports)]

#[macro_use]
extern crate bitflags;

extern crate gio;
extern crate gio_sys;
#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;
extern crate gtk_sys;
extern crate soup_sys;
extern crate libc;

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

mod auto;

pub use glib::Error;

pub use auto::*;