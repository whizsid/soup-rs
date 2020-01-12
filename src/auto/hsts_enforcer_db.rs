// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_68", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use soup_sys;
use std::fmt;
use HSTSEnforcer;

glib_wrapper! {
    pub struct HSTSEnforcerDB(Object<soup_sys::SoupHSTSEnforcerDB, soup_sys::SoupHSTSEnforcerDBClass, HSTSEnforcerDBClass>) @extends HSTSEnforcer;

    match fn {
        get_type => || soup_sys::soup_hsts_enforcer_db_get_type(),
    }
}

impl HSTSEnforcerDB {
    #[cfg(any(feature = "v2_68", feature = "dox"))]
    pub fn new(filename: &str) -> HSTSEnforcerDB {
        assert_initialized_main_thread!();
        unsafe {
            HSTSEnforcer::from_glib_full(soup_sys::soup_hsts_enforcer_db_new(filename.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_HSTS_ENFORCER_DB: Option<&HSTSEnforcerDB> = None;

pub trait HSTSEnforcerDBExt: 'static {
    fn get_property_filename(&self) -> Option<GString>;
}

impl<O: IsA<HSTSEnforcerDB>> HSTSEnforcerDBExt for O {
    fn get_property_filename(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filename\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `filename` getter")
        }
    }
}

impl fmt::Display for HSTSEnforcerDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HSTSEnforcerDB")
    }
}