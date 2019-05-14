// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use CellAccessible;
use CellRenderer;
use RendererCellAccessible;
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
    pub struct BooleanCellAccessible(Object<gtk_sys::GtkBooleanCellAccessible, gtk_sys::GtkBooleanCellAccessibleClass, BooleanCellAccessibleClass>) @extends RendererCellAccessible, CellAccessible, Accessible, atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_boolean_cell_accessible_get_type(),
    }
}

impl BooleanCellAccessible {}

pub const NONE_BOOLEAN_CELL_ACCESSIBLE: Option<&BooleanCellAccessible> = None;

impl fmt::Display for BooleanCellAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BooleanCellAccessible")
    }
}
