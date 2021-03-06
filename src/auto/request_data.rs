// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use soup_sys;
use std::fmt;
use Request;

glib_wrapper! {
    pub struct RequestData(Object<soup_sys::SoupRequestData, soup_sys::SoupRequestDataClass, RequestDataClass>) @extends Request;

    match fn {
        get_type => || soup_sys::soup_request_data_get_type(),
    }
}

impl RequestData {}

pub const NONE_REQUEST_DATA: Option<&RequestData> = None;

impl fmt::Display for RequestData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RequestData")
    }
}
