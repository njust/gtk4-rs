// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
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
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Statusbar(Object<gtk_sys::GtkStatusbar, gtk_sys::GtkStatusbarClass, StatusbarClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_statusbar_get_type(),
    }
}

impl Statusbar {
    pub fn new() -> Statusbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_statusbar_new()).unsafe_cast()
        }
    }
}

impl Default for Statusbar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STATUSBAR: Option<&Statusbar> = None;

pub trait StatusbarExt: 'static {
    fn get_context_id(&self, context_description: &str) -> u32;

    fn get_message_area(&self) -> Option<Box>;

    fn pop(&self, context_id: u32);

    fn push(&self, context_id: u32, text: &str) -> u32;

    fn remove(&self, context_id: u32, message_id: u32);

    fn remove_all(&self, context_id: u32);

    fn connect_text_popped<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_pushed<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Statusbar>> StatusbarExt for O {
    fn get_context_id(&self, context_description: &str) -> u32 {
        unsafe {
            gtk_sys::gtk_statusbar_get_context_id(self.as_ref().to_glib_none().0, context_description.to_glib_none().0)
        }
    }

    fn get_message_area(&self) -> Option<Box> {
        unsafe {
            from_glib_none(gtk_sys::gtk_statusbar_get_message_area(self.as_ref().to_glib_none().0))
        }
    }

    fn pop(&self, context_id: u32) {
        unsafe {
            gtk_sys::gtk_statusbar_pop(self.as_ref().to_glib_none().0, context_id);
        }
    }

    fn push(&self, context_id: u32, text: &str) -> u32 {
        unsafe {
            gtk_sys::gtk_statusbar_push(self.as_ref().to_glib_none().0, context_id, text.to_glib_none().0)
        }
    }

    fn remove(&self, context_id: u32, message_id: u32) {
        unsafe {
            gtk_sys::gtk_statusbar_remove(self.as_ref().to_glib_none().0, context_id, message_id);
        }
    }

    fn remove_all(&self, context_id: u32) {
        unsafe {
            gtk_sys::gtk_statusbar_remove_all(self.as_ref().to_glib_none().0, context_id);
        }
    }

    fn connect_text_popped<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"text-popped\0".as_ptr() as *const _,
                Some(transmute(text_popped_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_text_pushed<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"text-pushed\0".as_ptr() as *const _,
                Some(transmute(text_pushed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn text_popped_trampoline<P, F: Fn(&P, u32, &str) + 'static>(this: *mut gtk_sys::GtkStatusbar, context_id: libc::c_uint, text: *mut libc::c_char, f: glib_sys::gpointer)
where P: IsA<Statusbar> {
    let f: &F = &*(f as *const F);
    f(&Statusbar::from_glib_borrow(this).unsafe_cast(), context_id, &GString::from_glib_borrow(text))
}

unsafe extern "C" fn text_pushed_trampoline<P, F: Fn(&P, u32, &str) + 'static>(this: *mut gtk_sys::GtkStatusbar, context_id: libc::c_uint, text: *mut libc::c_char, f: glib_sys::gpointer)
where P: IsA<Statusbar> {
    let f: &F = &*(f as *const F);
    f(&Statusbar::from_glib_borrow(this).unsafe_cast(), context_id, &GString::from_glib_borrow(text))
}

impl fmt::Display for Statusbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Statusbar")
    }
}
