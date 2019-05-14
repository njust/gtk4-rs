// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use Orientable;
use Widget;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::ObjectExt;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ShortcutsSection(Object<gtk_sys::GtkShortcutsSection, gtk_sys::GtkShortcutsSectionClass, ShortcutsSectionClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_shortcuts_section_get_type(),
    }
}

impl ShortcutsSection {
    pub fn get_property_max_height(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"max-height\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_max_height(&self, max_height: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"max-height\0".as_ptr() as *const _, Value::from(&max_height).to_glib_none().0);
        }
    }

    pub fn get_property_section_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"section-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_section_name(&self, section_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"section-name\0".as_ptr() as *const _, Value::from(section_name).to_glib_none().0);
        }
    }

    pub fn get_property_title(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"title\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_title(&self, title: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"title\0".as_ptr() as *const _, Value::from(title).to_glib_none().0);
        }
    }

    pub fn get_property_view_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"view-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_view_name(&self, view_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"view-name\0".as_ptr() as *const _, Value::from(view_name).to_glib_none().0);
        }
    }

    pub fn connect_change_current_page<F: Fn(&ShortcutsSection, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"change-current-page\0".as_ptr() as *const _,
                Some(transmute(change_current_page_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn emit_change_current_page(&self, object: i32) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("change-current-page", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    pub fn connect_property_max_height_notify<F: Fn(&ShortcutsSection) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-height\0".as_ptr() as *const _,
                Some(transmute(notify_max_height_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_section_name_notify<F: Fn(&ShortcutsSection) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::section-name\0".as_ptr() as *const _,
                Some(transmute(notify_section_name_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_title_notify<F: Fn(&ShortcutsSection) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_view_name_notify<F: Fn(&ShortcutsSection) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::view-name\0".as_ptr() as *const _,
                Some(transmute(notify_view_name_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn change_current_page_trampoline<F: Fn(&ShortcutsSection, i32) -> bool + 'static>(this: *mut gtk_sys::GtkShortcutsSection, object: libc::c_int, f: glib_sys::gpointer) -> glib_sys::gboolean {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), object).to_glib()
}

unsafe extern "C" fn notify_max_height_trampoline<F: Fn(&ShortcutsSection) + 'static>(this: *mut gtk_sys::GtkShortcutsSection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_section_name_trampoline<F: Fn(&ShortcutsSection) + 'static>(this: *mut gtk_sys::GtkShortcutsSection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_title_trampoline<F: Fn(&ShortcutsSection) + 'static>(this: *mut gtk_sys::GtkShortcutsSection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_view_name_trampoline<F: Fn(&ShortcutsSection) + 'static>(this: *mut gtk_sys::GtkShortcutsSection, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

impl fmt::Display for ShortcutsSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShortcutsSection")
    }
}
