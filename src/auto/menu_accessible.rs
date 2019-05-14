// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use ContainerAccessible;
use MenuShellAccessible;
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
    pub struct MenuAccessible(Object<gtk_sys::GtkMenuAccessible, gtk_sys::GtkMenuAccessibleClass, MenuAccessibleClass>) @extends MenuShellAccessible, ContainerAccessible, WidgetAccessible, Accessible, atk::Object;

    match fn {
        get_type => || gtk_sys::gtk_menu_accessible_get_type(),
    }
}

impl MenuAccessible {}

pub const NONE_MENU_ACCESSIBLE: Option<&MenuAccessible> = None;

impl fmt::Display for MenuAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuAccessible")
    }
}
