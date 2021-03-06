// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use soup_sys;
use std::fmt;
use Auth;

glib_wrapper! {
    pub struct AuthNTLM(Object<soup_sys::SoupAuthNTLM, AuthNTLMClass>) @extends Auth;

    match fn {
        get_type => || soup_sys::soup_auth_ntlm_get_type(),
    }
}

impl AuthNTLM {}

impl fmt::Display for AuthNTLM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthNTLM")
    }
}
