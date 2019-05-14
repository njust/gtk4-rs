// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
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
    pub struct Bin(Object<gtk_sys::GtkBin, gtk_sys::GtkBinClass, BinClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_bin_get_type(),
    }
}

pub const NONE_BIN: Option<&Bin> = None;

pub trait BinExt: 'static {
    fn get_child(&self) -> Option<Widget>;
}

impl<O: IsA<Bin>> BinExt for O {
    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_bin_get_child(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for Bin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bin")
    }
}
