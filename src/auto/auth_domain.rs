// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use soup_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Message;

glib_wrapper! {
    pub struct AuthDomain(Object<soup_sys::SoupAuthDomain, soup_sys::SoupAuthDomainClass, AuthDomainClass>);

    match fn {
        get_type => || soup_sys::soup_auth_domain_get_type(),
    }
}

pub const NONE_AUTH_DOMAIN: Option<&AuthDomain> = None;

pub trait AuthDomainExt: 'static {
    fn accepts<P: IsA<Message>>(&self, msg: &P) -> Option<GString>;

    fn add_path(&self, path: &str);

    fn challenge<P: IsA<Message>>(&self, msg: &P);

    fn check_password<P: IsA<Message>>(&self, msg: &P, username: &str, password: &str) -> bool;

    fn covers<P: IsA<Message>>(&self, msg: &P) -> bool;

    fn get_realm(&self) -> Option<GString>;

    fn remove_path(&self, path: &str);

    fn try_generic_auth_callback<P: IsA<Message>>(&self, msg: &P, username: &str) -> bool;

    fn set_property_add_path(&self, add_path: Option<&str>);

    fn get_property_filter(&self) -> Fn(&AuthDomain, &Message) -> bool + 'static;

    fn set_property_filter(&self, filter: Fn(&AuthDomain, &Message) -> bool + 'static);

    //fn get_property_filter_data(&self) -> /*Unimplemented*/Fundamental: Pointer;

    //fn set_property_filter_data(&self, filter_data: /*Unimplemented*/Fundamental: Pointer);

    fn get_property_generic_auth_callback(&self) -> Fn(&AuthDomain, &Message, &str) -> bool + 'static;

    fn set_property_generic_auth_callback(&self, generic_auth_callback: Fn(&AuthDomain, &Message, &str) -> bool + 'static);

    //fn get_property_generic_auth_data(&self) -> /*Unimplemented*/Fundamental: Pointer;

    //fn set_property_generic_auth_data(&self, generic_auth_data: /*Unimplemented*/Fundamental: Pointer);

    fn get_property_proxy(&self) -> bool;

    fn set_property_remove_path(&self, remove_path: Option<&str>);

    fn connect_property_add_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filter_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_generic_auth_callback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_generic_auth_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_remove_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthDomain>> AuthDomainExt for O {
    fn accepts<P: IsA<Message>>(&self, msg: &P) -> Option<GString> {
        unsafe {
            from_glib_full(soup_sys::soup_auth_domain_accepts(self.as_ref().to_glib_none().0, msg.as_ref().to_glib_none().0))
        }
    }

    fn add_path(&self, path: &str) {
        unsafe {
            soup_sys::soup_auth_domain_add_path(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn challenge<P: IsA<Message>>(&self, msg: &P) {
        unsafe {
            soup_sys::soup_auth_domain_challenge(self.as_ref().to_glib_none().0, msg.as_ref().to_glib_none().0);
        }
    }

    fn check_password<P: IsA<Message>>(&self, msg: &P, username: &str, password: &str) -> bool {
        unsafe {
            from_glib(soup_sys::soup_auth_domain_check_password(self.as_ref().to_glib_none().0, msg.as_ref().to_glib_none().0, username.to_glib_none().0, password.to_glib_none().0))
        }
    }

    fn covers<P: IsA<Message>>(&self, msg: &P) -> bool {
        unsafe {
            from_glib(soup_sys::soup_auth_domain_covers(self.as_ref().to_glib_none().0, msg.as_ref().to_glib_none().0))
        }
    }

    fn get_realm(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_auth_domain_get_realm(self.as_ref().to_glib_none().0))
        }
    }

    fn remove_path(&self, path: &str) {
        unsafe {
            soup_sys::soup_auth_domain_remove_path(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn try_generic_auth_callback<P: IsA<Message>>(&self, msg: &P, username: &str) -> bool {
        unsafe {
            from_glib(soup_sys::soup_auth_domain_try_generic_auth_callback(self.as_ref().to_glib_none().0, msg.as_ref().to_glib_none().0, username.to_glib_none().0))
        }
    }

    fn set_property_add_path(&self, add_path: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"add-path\0".as_ptr() as *const _, Value::from(add_path).to_glib_none().0);
        }
    }

    fn get_property_filter(&self) -> Fn(&AuthDomain, &Message) -> bool + 'static {
        unsafe {
            let mut value = Value::from_type(<Fn(&AuthDomain, &Message) -> bool + 'static as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `filter` getter").unwrap()
        }
    }

    fn set_property_filter(&self, filter: Fn(&AuthDomain, &Message) -> bool + 'static) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter\0".as_ptr() as *const _, Value::from(&filter).to_glib_none().0);
        }
    }

    //fn get_property_filter_data(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter-data\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `filter-data` getter").unwrap()
    //    }
    //}

    //fn set_property_filter_data(&self, filter_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter-data\0".as_ptr() as *const _, Value::from(&filter_data).to_glib_none().0);
    //    }
    //}

    fn get_property_generic_auth_callback(&self) -> Fn(&AuthDomain, &Message, &str) -> bool + 'static {
        unsafe {
            let mut value = Value::from_type(<Fn(&AuthDomain, &Message, &str) -> bool + 'static as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"generic-auth-callback\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `generic-auth-callback` getter").unwrap()
        }
    }

    fn set_property_generic_auth_callback(&self, generic_auth_callback: Fn(&AuthDomain, &Message, &str) -> bool + 'static) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"generic-auth-callback\0".as_ptr() as *const _, Value::from(&generic_auth_callback).to_glib_none().0);
        }
    }

    //fn get_property_generic_auth_data(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"generic-auth-data\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `generic-auth-data` getter").unwrap()
    //    }
    //}

    //fn set_property_generic_auth_data(&self, generic_auth_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"generic-auth-data\0".as_ptr() as *const _, Value::from(&generic_auth_data).to_glib_none().0);
    //    }
    //}

    fn get_property_proxy(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"proxy\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `proxy` getter").unwrap()
        }
    }

    fn set_property_remove_path(&self, remove_path: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"remove-path\0".as_ptr() as *const _, Value::from(remove_path).to_glib_none().0);
        }
    }

    fn connect_property_add_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_add_path_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAuthDomain, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AuthDomain>
        {
            let f: &F = &*(f as *const F);
            f(&AuthDomain::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::add-path\0".as_ptr() as *const _,
                Some(transmute(notify_add_path_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAuthDomain, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AuthDomain>
        {
            let f: &F = &*(f as *const F);
            f(&AuthDomain::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::filter\0".as_ptr() as *const _,
                Some(transmute(notify_filter_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_filter_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_data_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAuthDomain, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AuthDomain>
        {
            let f: &F = &*(f as *const F);
            f(&AuthDomain::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::filter-data\0".as_ptr() as *const _,
                Some(transmute(notify_filter_data_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_generic_auth_callback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_generic_auth_callback_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAuthDomain, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AuthDomain>
        {
            let f: &F = &*(f as *const F);
            f(&AuthDomain::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::generic-auth-callback\0".as_ptr() as *const _,
                Some(transmute(notify_generic_auth_callback_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_generic_auth_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_generic_auth_data_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAuthDomain, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AuthDomain>
        {
            let f: &F = &*(f as *const F);
            f(&AuthDomain::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::generic-auth-data\0".as_ptr() as *const _,
                Some(transmute(notify_generic_auth_data_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_remove_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_remove_path_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupAuthDomain, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AuthDomain>
        {
            let f: &F = &*(f as *const F);
            f(&AuthDomain::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::remove-path\0".as_ptr() as *const _,
                Some(transmute(notify_remove_path_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for AuthDomain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthDomain")
    }
}
