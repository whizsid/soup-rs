// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use soup_sys;

bitflags! {
    pub struct Cacheability: u32 {
        const CACHEABLE = 1;
        const UNCACHEABLE = 2;
        const INVALIDATES = 4;
        const VALIDATES = 8;
    }
}

#[doc(hidden)]
impl ToGlib for Cacheability {
    type GlibType = soup_sys::SoupCacheability;

    fn to_glib(&self) -> soup_sys::SoupCacheability {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<soup_sys::SoupCacheability> for Cacheability {
    fn from_glib(value: soup_sys::SoupCacheability) -> Cacheability {
        skip_assert_initialized!();
        Cacheability::from_bits_truncate(value)
    }
}

impl StaticType for Cacheability {
    fn static_type() -> Type {
        unsafe { from_glib(soup_sys::soup_cacheability_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Cacheability {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Cacheability {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for Cacheability {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct Expectation: u32 {
        const UNRECOGNIZED = 1;
        const CONTINUE = 2;
    }
}

#[doc(hidden)]
impl ToGlib for Expectation {
    type GlibType = soup_sys::SoupExpectation;

    fn to_glib(&self) -> soup_sys::SoupExpectation {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<soup_sys::SoupExpectation> for Expectation {
    fn from_glib(value: soup_sys::SoupExpectation) -> Expectation {
        skip_assert_initialized!();
        Expectation::from_bits_truncate(value)
    }
}

impl StaticType for Expectation {
    fn static_type() -> Type {
        unsafe { from_glib(soup_sys::soup_expectation_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Expectation {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Expectation {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for Expectation {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct MessageFlags: u32 {
        const NO_REDIRECT = 2;
        const CAN_REBUILD = 4;
        const OVERWRITE_CHUNKS = 8;
        const CONTENT_DECODED = 16;
        const CERTIFICATE_TRUSTED = 32;
        const NEW_CONNECTION = 64;
        const IDEMPOTENT = 128;
        const IGNORE_CONNECTION_LIMITS = 256;
        const DO_NOT_USE_AUTH_CACHE = 512;
    }
}

#[doc(hidden)]
impl ToGlib for MessageFlags {
    type GlibType = soup_sys::SoupMessageFlags;

    fn to_glib(&self) -> soup_sys::SoupMessageFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<soup_sys::SoupMessageFlags> for MessageFlags {
    fn from_glib(value: soup_sys::SoupMessageFlags) -> MessageFlags {
        skip_assert_initialized!();
        MessageFlags::from_bits_truncate(value)
    }
}

impl StaticType for MessageFlags {
    fn static_type() -> Type {
        unsafe { from_glib(soup_sys::soup_message_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MessageFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MessageFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for MessageFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
bitflags! {
    pub struct ServerListenOptions: u32 {
        const HTTPS = 1;
        const IPV4_ONLY = 2;
        const IPV6_ONLY = 4;
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for ServerListenOptions {
    type GlibType = soup_sys::SoupServerListenOptions;

    fn to_glib(&self) -> soup_sys::SoupServerListenOptions {
        self.bits()
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<soup_sys::SoupServerListenOptions> for ServerListenOptions {
    fn from_glib(value: soup_sys::SoupServerListenOptions) -> ServerListenOptions {
        skip_assert_initialized!();
        ServerListenOptions::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
impl StaticType for ServerListenOptions {
    fn static_type() -> Type {
        unsafe { from_glib(soup_sys::soup_server_listen_options_get_type()) }
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
impl<'a> FromValueOptional<'a> for ServerListenOptions {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
impl<'a> FromValue<'a> for ServerListenOptions {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
impl SetValue for ServerListenOptions {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

