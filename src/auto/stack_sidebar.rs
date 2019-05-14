// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Stack;
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
    pub struct StackSidebar(Object<gtk_sys::GtkStackSidebar, gtk_sys::GtkStackSidebarClass, StackSidebarClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_stack_sidebar_get_type(),
    }
}

impl StackSidebar {
    pub fn new() -> StackSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_stack_sidebar_new()).unsafe_cast()
        }
    }
}

impl Default for StackSidebar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STACK_SIDEBAR: Option<&StackSidebar> = None;

pub trait StackSidebarExt: 'static {
    fn get_stack(&self) -> Option<Stack>;

    fn set_stack<P: IsA<Stack>>(&self, stack: &P);

    fn connect_property_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StackSidebar>> StackSidebarExt for O {
    fn get_stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_none(gtk_sys::gtk_stack_sidebar_get_stack(self.as_ref().to_glib_none().0))
        }
    }

    fn set_stack<P: IsA<Stack>>(&self, stack: &P) {
        unsafe {
            gtk_sys::gtk_stack_sidebar_set_stack(self.as_ref().to_glib_none().0, stack.as_ref().to_glib_none().0);
        }
    }

    fn connect_property_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stack\0".as_ptr() as *const _,
                Some(transmute(notify_stack_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_stack_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStackSidebar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StackSidebar> {
    let f: &F = &*(f as *const F);
    f(&StackSidebar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for StackSidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StackSidebar")
    }
}
