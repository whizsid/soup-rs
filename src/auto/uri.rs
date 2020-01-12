// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use soup_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct URI(Boxed<soup_sys::SoupURI>);

    match fn {
        copy => |ptr| soup_sys::soup_uri_copy(mut_override(ptr)),
        free => |ptr| soup_sys::soup_uri_free(ptr),
        get_type => || soup_sys::soup_uri_get_type(),
    }
}

impl URI {
    pub fn new(uri_string: Option<&str>) -> Option<URI> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_uri_new(uri_string.to_glib_none().0))
        }
    }

    pub fn new_with_base(base: &mut URI, uri_string: &str) -> URI {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_uri_new_with_base(base.to_glib_none_mut().0, uri_string.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn copy_host(&mut self) -> Option<URI> {
        unsafe {
            from_glib_full(soup_sys::soup_uri_copy_host(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_fragment(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_fragment(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_host(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_host(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_password(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_password(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_path(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_path(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_port(&mut self) -> u32 {
        unsafe {
            soup_sys::soup_uri_get_port(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_query(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_query(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_scheme(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_scheme(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_user(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_uri_get_user(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn host_equal(&self, v2: &URI) -> bool {
        unsafe {
            from_glib(soup_sys::soup_uri_host_equal(self.to_glib_none().0, v2.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn host_hash(&self) -> u32 {
        unsafe {
            soup_sys::soup_uri_host_hash(self.to_glib_none().0)
        }
    }

    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        unsafe {
            soup_sys::soup_uri_set_fragment(self.to_glib_none_mut().0, fragment.to_glib_none().0);
        }
    }

    pub fn set_host(&mut self, host: Option<&str>) {
        unsafe {
            soup_sys::soup_uri_set_host(self.to_glib_none_mut().0, host.to_glib_none().0);
        }
    }

    pub fn set_password(&mut self, password: Option<&str>) {
        unsafe {
            soup_sys::soup_uri_set_password(self.to_glib_none_mut().0, password.to_glib_none().0);
        }
    }

    pub fn set_path(&mut self, path: &str) {
        unsafe {
            soup_sys::soup_uri_set_path(self.to_glib_none_mut().0, path.to_glib_none().0);
        }
    }

    pub fn set_port(&mut self, port: u32) {
        unsafe {
            soup_sys::soup_uri_set_port(self.to_glib_none_mut().0, port);
        }
    }

    pub fn set_query(&mut self, query: Option<&str>) {
        unsafe {
            soup_sys::soup_uri_set_query(self.to_glib_none_mut().0, query.to_glib_none().0);
        }
    }

    //pub fn set_query_from_fields(&mut self, first_field: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call soup_sys:soup_uri_set_query_from_fields() }
    //}

    //pub fn set_query_from_form(&mut self, form: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
    //    unsafe { TODO: call soup_sys:soup_uri_set_query_from_form() }
    //}

    pub fn set_scheme(&mut self, scheme: &str) {
        unsafe {
            soup_sys::soup_uri_set_scheme(self.to_glib_none_mut().0, scheme.to_glib_none().0);
        }
    }

    pub fn set_user(&mut self, user: Option<&str>) {
        unsafe {
            soup_sys::soup_uri_set_user(self.to_glib_none_mut().0, user.to_glib_none().0);
        }
    }

    pub fn to_string(&mut self, just_path_and_query: bool) -> GString {
        unsafe {
            from_glib_full(soup_sys::soup_uri_to_string(self.to_glib_none_mut().0, just_path_and_query.to_glib()))
        }
    }

    pub fn uses_default_port(&mut self) -> bool {
        unsafe {
            from_glib(soup_sys::soup_uri_uses_default_port(self.to_glib_none_mut().0))
        }
    }

    pub fn decode(part: &str) -> Option<GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_uri_decode(part.to_glib_none().0))
        }
    }

    pub fn encode(part: &str, escape_extra: Option<&str>) -> Option<GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_uri_encode(part.to_glib_none().0, escape_extra.to_glib_none().0))
        }
    }

    pub fn normalize(part: &str, unescape_extra: Option<&str>) -> Option<GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_uri_normalize(part.to_glib_none().0, unescape_extra.to_glib_none().0))
        }
    }
}
