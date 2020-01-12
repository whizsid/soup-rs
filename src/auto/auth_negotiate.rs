// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use soup_sys;
use std::fmt;
use Auth;

glib_wrapper! {
    pub struct AuthNegotiate(Object<soup_sys::SoupAuthNegotiate, AuthNegotiateClass>) @extends Auth;

    match fn {
        get_type => || soup_sys::soup_auth_negotiate_get_type(),
    }
}

impl AuthNegotiate {
    #[cfg(any(feature = "v2_54", feature = "dox"))]
    pub fn supported() -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(soup_sys::soup_auth_negotiate_supported())
        }
    }
}

impl fmt::Display for AuthNegotiate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthNegotiate")
    }
}