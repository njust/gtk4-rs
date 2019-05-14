// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use WidgetAccessible;
use atk;
use atk_sys;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ScrolledWindowAccessible(Object<gtk_sys::GtkScrolledWindowAccessible, gtk_sys::GtkScrolledWindowAccessibleClass, ScrolledWindowAccessibleClass>) @extends ContainerAccessible, WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_scrolled_window_accessible_get_type(),
    }
}

impl ScrolledWindowAccessible {}

pub const NONE_SCROLLED_WINDOW_ACCESSIBLE: Option<&ScrolledWindowAccessible> = None;

impl fmt::Display for ScrolledWindowAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrolledWindowAccessible")
    }
}
