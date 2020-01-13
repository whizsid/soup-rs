// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_24", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v2_24", feature = "dox"))]
use glib::GString;
use soup_sys;
#[cfg(any(feature = "v2_24", feature = "dox"))]
use Date;
#[cfg(any(feature = "v2_24", feature = "dox"))]
use URI;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Cookie(Boxed<soup_sys::SoupCookie>);

    match fn {
        copy => |ptr| soup_sys::soup_cookie_copy(mut_override(ptr)),
        free => |ptr| soup_sys::soup_cookie_free(ptr),
        get_type => || soup_sys::soup_cookie_get_type(),
    }
}

impl Cookie {
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn new(name: &str, value: &str, domain: &str, path: &str, max_age: i32) -> Cookie {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_cookie_new(name.to_glib_none().0, value.to_glib_none().0, domain.to_glib_none().0, path.to_glib_none().0, max_age))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn applies_to_uri(&mut self, uri: &mut URI) -> bool {
        unsafe {
            from_glib(soup_sys::soup_cookie_applies_to_uri(self.to_glib_none_mut().0, uri.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    pub fn domain_matches(&mut self, host: &str) -> bool {
        unsafe {
            from_glib(soup_sys::soup_cookie_domain_matches(self.to_glib_none_mut().0, host.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_domain(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_cookie_get_domain(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_expires(&mut self) -> Option<Date> {
        unsafe {
            from_glib_none(soup_sys::soup_cookie_get_expires(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_http_only(&mut self) -> bool {
        unsafe {
            from_glib(soup_sys::soup_cookie_get_http_only(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_name(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_cookie_get_name(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_path(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_cookie_get_path(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_secure(&mut self) -> bool {
        unsafe {
            from_glib(soup_sys::soup_cookie_get_secure(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_value(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_cookie_get_value(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_domain(&mut self, domain: &str) {
        unsafe {
            soup_sys::soup_cookie_set_domain(self.to_glib_none_mut().0, domain.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_expires(&mut self, expires: &mut Date) {
        unsafe {
            soup_sys::soup_cookie_set_expires(self.to_glib_none_mut().0, expires.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_http_only(&mut self, http_only: bool) {
        unsafe {
            soup_sys::soup_cookie_set_http_only(self.to_glib_none_mut().0, http_only.to_glib());
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_max_age(&mut self, max_age: i32) {
        unsafe {
            soup_sys::soup_cookie_set_max_age(self.to_glib_none_mut().0, max_age);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_name(&mut self, name: &str) {
        unsafe {
            soup_sys::soup_cookie_set_name(self.to_glib_none_mut().0, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_path(&mut self, path: &str) {
        unsafe {
            soup_sys::soup_cookie_set_path(self.to_glib_none_mut().0, path.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_secure(&mut self, secure: bool) {
        unsafe {
            soup_sys::soup_cookie_set_secure(self.to_glib_none_mut().0, secure.to_glib());
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn set_value(&mut self, value: &str) {
        unsafe {
            soup_sys::soup_cookie_set_value(self.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn to_cookie_header(&mut self) -> Option<GString> {
        unsafe {
            from_glib_full(soup_sys::soup_cookie_to_cookie_header(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn to_set_cookie_header(&mut self) -> Option<GString> {
        unsafe {
            from_glib_full(soup_sys::soup_cookie_to_set_cookie_header(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn parse(header: &str, origin: &mut URI) -> Option<Cookie> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_cookie_parse(header.to_glib_none().0, origin.to_glib_none_mut().0))
        }
    }
}