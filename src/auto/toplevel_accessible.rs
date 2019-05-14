// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Window;
use atk;
use atk_sys;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
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
    pub struct ToplevelAccessible(Object<gtk_sys::GtkToplevelAccessible, gtk_sys::GtkToplevelAccessibleClass, ToplevelAccessibleClass>) @extends atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_toplevel_accessible_get_type(),
    }
}

pub const NONE_TOPLEVEL_ACCESSIBLE: Option<&ToplevelAccessible> = None;

pub trait ToplevelAccessibleExt: 'static {
    fn get_children(&self) -> Vec<Window>;
}

impl<O: IsA<ToplevelAccessible>> ToplevelAccessibleExt for O {
    fn get_children(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_sys::gtk_toplevel_accessible_get_children(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for ToplevelAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToplevelAccessible")
    }
}
