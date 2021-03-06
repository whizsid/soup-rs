// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use soup_sys;
use std::fmt;
use CacheType;

glib_wrapper! {
    pub struct Cache(Object<soup_sys::SoupCache, soup_sys::SoupCacheClass, CacheClass>);

    match fn {
        get_type => || soup_sys::soup_cache_get_type(),
    }
}

impl Cache {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    pub fn new(cache_dir: Option<&str>, cache_type: CacheType) -> Cache {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_cache_new(cache_dir.to_glib_none().0, cache_type.to_glib()))
        }
    }
}

pub const NONE_CACHE: Option<&Cache> = None;

pub trait CacheExt: 'static {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn clear(&self);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn dump(&self);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn flush(&self);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_max_size(&self) -> u32;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn load(&self);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn set_max_size(&self, max_size: u32);

    fn get_property_cache_dir(&self) -> Option<GString>;

    fn get_property_cache_type(&self) -> CacheType;
}

impl<O: IsA<Cache>> CacheExt for O {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn clear(&self) {
        unsafe {
            soup_sys::soup_cache_clear(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn dump(&self) {
        unsafe {
            soup_sys::soup_cache_dump(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn flush(&self) {
        unsafe {
            soup_sys::soup_cache_flush(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_max_size(&self) -> u32 {
        unsafe {
            soup_sys::soup_cache_get_max_size(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn load(&self) {
        unsafe {
            soup_sys::soup_cache_load(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn set_max_size(&self, max_size: u32) {
        unsafe {
            soup_sys::soup_cache_set_max_size(self.as_ref().to_glib_none().0, max_size);
        }
    }

    fn get_property_cache_dir(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"cache-dir\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `cache-dir` getter")
        }
    }

    fn get_property_cache_type(&self) -> CacheType {
        unsafe {
            let mut value = Value::from_type(<CacheType as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"cache-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `cache-type` getter").unwrap()
        }
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cache")
    }
}
