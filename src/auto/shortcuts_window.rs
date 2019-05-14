// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Root;
use Widget;
use Window;
use WindowPosition;
use WindowType;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    pub struct ShortcutsWindow(Object<gtk_sys::GtkShortcutsWindow, gtk_sys::GtkShortcutsWindowClass, ShortcutsWindowClass>) @extends Window, Bin, Container, Widget, @implements Buildable, Root;

    match fn {
        get_type => || gtk_sys::gtk_shortcuts_window_get_type(),
    }
}

pub const NONE_SHORTCUTS_WINDOW: Option<&ShortcutsWindow> = None;

pub trait ShortcutsWindowExt: 'static {
    fn get_property_section_name(&self) -> Option<GString>;

    fn set_property_section_name(&self, section_name: Option<&str>);

    fn get_property_view_name(&self) -> Option<GString>;

    fn set_property_view_name(&self, view_name: Option<&str>);

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_close(&self);

    fn connect_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_search(&self);

    fn connect_property_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ShortcutsWindow>> ShortcutsWindowExt for O {
    fn get_property_section_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"section-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_section_name(&self, section_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"section-name\0".as_ptr() as *const _, Value::from(section_name).to_glib_none().0);
        }
    }

    fn get_property_view_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"view-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_view_name(&self, view_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"view-name\0".as_ptr() as *const _, Value::from(view_name).to_glib_none().0);
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"close\0".as_ptr() as *const _,
                Some(transmute(close_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_close(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("close", &[]).unwrap() };
    }

    fn connect_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"search\0".as_ptr() as *const _,
                Some(transmute(search_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_search(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("search", &[]).unwrap() };
    }

    fn connect_property_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::section-name\0".as_ptr() as *const _,
                Some(transmute(notify_section_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::view-name\0".as_ptr() as *const _,
                Some(transmute(notify_view_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn close_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkShortcutsWindow, f: glib_sys::gpointer)
where P: IsA<ShortcutsWindow> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsWindow::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn search_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkShortcutsWindow, f: glib_sys::gpointer)
where P: IsA<ShortcutsWindow> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsWindow::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_section_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkShortcutsWindow, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ShortcutsWindow> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsWindow::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_view_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkShortcutsWindow, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ShortcutsWindow> {
    let f: &F = &*(f as *const F);
    f(&ShortcutsWindow::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ShortcutsWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShortcutsWindow")
    }
}
