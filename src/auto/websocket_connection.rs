// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_50", feature = "dox"))]
use glib;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use libc;
use soup_sys;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use URI;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use WebsocketConnectionType;
#[cfg(any(feature = "v2_68", feature = "dox"))]
use WebsocketDataType;
#[cfg(any(feature = "v2_68", feature = "dox"))]
use WebsocketExtension;
#[cfg(any(feature = "v2_50", feature = "dox"))]
use WebsocketState;

glib_wrapper! {
    pub struct WebsocketConnection(Object<soup_sys::SoupWebsocketConnection, soup_sys::SoupWebsocketConnectionClass, WebsocketConnectionClass>);

    match fn {
        get_type => || soup_sys::soup_websocket_connection_get_type(),
    }
}

impl WebsocketConnection {
    //#[cfg(any(feature = "v2_50", feature = "dox"))]
    //pub fn new(stream: /*Ignored*/&gio::IOStream, uri: &mut URI, type_: WebsocketConnectionType, origin: Option<&str>, protocol: Option<&str>) -> WebsocketConnection {
    //    unsafe { TODO: call soup_sys:soup_websocket_connection_new() }
    //}

    //#[cfg(any(feature = "v2_68", feature = "dox"))]
    //pub fn new_with_extensions(stream: /*Ignored*/&gio::IOStream, uri: &mut URI, type_: WebsocketConnectionType, origin: Option<&str>, protocol: Option<&str>, extensions: &[WebsocketExtension]) -> WebsocketConnection {
    //    unsafe { TODO: call soup_sys:soup_websocket_connection_new_with_extensions() }
    //}
}

pub const NONE_WEBSOCKET_CONNECTION: Option<&WebsocketConnection> = None;

pub trait WebsocketConnectionExt: 'static {
    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn close(&self, code: libc::c_ushort, data: Option<&str>);

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_close_code(&self) -> libc::c_ushort;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_close_data(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_connection_type(&self) -> WebsocketConnectionType;

    #[cfg(any(feature = "v2_68", feature = "dox"))]
    fn get_extensions(&self) -> Vec<WebsocketExtension>;

    //#[cfg(any(feature = "v2_50", feature = "dox"))]
    //fn get_io_stream(&self) -> /*Ignored*/Option<gio::IOStream>;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_keepalive_interval(&self) -> u32;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn get_max_incoming_payload_size(&self) -> u64;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_origin(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_protocol(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_state(&self) -> WebsocketState;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_uri(&self) -> Option<URI>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn send_binary(&self, data: &[u8]);

    #[cfg(any(feature = "v2_68", feature = "dox"))]
    fn send_message(&self, type_: WebsocketDataType, message: &glib::Bytes);

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn send_text(&self, text: &str);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_keepalive_interval(&self, interval: u32);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_max_incoming_payload_size(&self, max_incoming_payload_size: u64);

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_closing<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_error<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_message<F: Fn(&Self, i32, &glib::Bytes) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn connect_pong<F: Fn(&Self, &glib::Bytes) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_keepalive_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn connect_property_max_incoming_payload_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebsocketConnection>> WebsocketConnectionExt for O {
    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn close(&self, code: libc::c_ushort, data: Option<&str>) {
        unsafe {
            soup_sys::soup_websocket_connection_close(self.as_ref().to_glib_none().0, code, data.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_close_code(&self) -> libc::c_ushort {
        unsafe {
            soup_sys::soup_websocket_connection_get_close_code(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_close_data(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_websocket_connection_get_close_data(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_connection_type(&self) -> WebsocketConnectionType {
        unsafe {
            from_glib(soup_sys::soup_websocket_connection_get_connection_type(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_68", feature = "dox"))]
    fn get_extensions(&self) -> Vec<WebsocketExtension> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(soup_sys::soup_websocket_connection_get_extensions(self.as_ref().to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_50", feature = "dox"))]
    //fn get_io_stream(&self) -> /*Ignored*/Option<gio::IOStream> {
    //    unsafe { TODO: call soup_sys:soup_websocket_connection_get_io_stream() }
    //}

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_keepalive_interval(&self) -> u32 {
        unsafe {
            soup_sys::soup_websocket_connection_get_keepalive_interval(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn get_max_incoming_payload_size(&self) -> u64 {
        unsafe {
            soup_sys::soup_websocket_connection_get_max_incoming_payload_size(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_origin(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_websocket_connection_get_origin(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_protocol(&self) -> Option<GString> {
        unsafe {
            from_glib_none(soup_sys::soup_websocket_connection_get_protocol(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_state(&self) -> WebsocketState {
        unsafe {
            from_glib(soup_sys::soup_websocket_connection_get_state(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_uri(&self) -> Option<URI> {
        unsafe {
            from_glib_none(soup_sys::soup_websocket_connection_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn send_binary(&self, data: &[u8]) {
        let length = data.len() as usize;
        unsafe {
            soup_sys::soup_websocket_connection_send_binary(self.as_ref().to_glib_none().0, data.to_glib_none().0, length);
        }
    }

    #[cfg(any(feature = "v2_68", feature = "dox"))]
    fn send_message(&self, type_: WebsocketDataType, message: &glib::Bytes) {
        unsafe {
            soup_sys::soup_websocket_connection_send_message(self.as_ref().to_glib_none().0, type_.to_glib(), message.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn send_text(&self, text: &str) {
        unsafe {
            soup_sys::soup_websocket_connection_send_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_keepalive_interval(&self, interval: u32) {
        unsafe {
            soup_sys::soup_websocket_connection_set_keepalive_interval(self.as_ref().to_glib_none().0, interval);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_max_incoming_payload_size(&self, max_incoming_payload_size: u64) {
        unsafe {
            soup_sys::soup_websocket_connection_set_max_incoming_payload_size(self.as_ref().to_glib_none().0, max_incoming_payload_size);
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"closed\0".as_ptr() as *const _,
                Some(transmute(closed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_closing<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closing_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"closing\0".as_ptr() as *const _,
                Some(transmute(closing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_error<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn error_trampoline<P, F: Fn(&P, &glib::Error) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, error: *mut glib_sys::GError, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"error\0".as_ptr() as *const _,
                Some(transmute(error_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_message<F: Fn(&Self, i32, &glib::Bytes) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn message_trampoline<P, F: Fn(&P, i32, &glib::Bytes) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, type_: libc::c_int, message: *mut glib_sys::GBytes, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast(), type_, &from_glib_borrow(message))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"message\0".as_ptr() as *const _,
                Some(transmute(message_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn connect_pong<F: Fn(&Self, &glib::Bytes) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pong_trampoline<P, F: Fn(&P, &glib::Bytes) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, message: *mut glib_sys::GBytes, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(message))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pong\0".as_ptr() as *const _,
                Some(transmute(pong_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_keepalive_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_keepalive_interval_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::keepalive-interval\0".as_ptr() as *const _,
                Some(transmute(notify_keepalive_interval_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn connect_property_max_incoming_payload_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_incoming_payload_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-incoming-payload-size\0".as_ptr() as *const _,
                Some(transmute(notify_max_incoming_payload_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(this: *mut soup_sys::SoupWebsocketConnection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<WebsocketConnection>
        {
            let f: &F = &*(f as *const F);
            f(&WebsocketConnection::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(transmute(notify_state_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for WebsocketConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WebsocketConnection")
    }
}
