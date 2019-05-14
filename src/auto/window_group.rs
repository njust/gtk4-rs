// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Widget;
use Window;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct WindowGroup(Object<gtk_sys::GtkWindowGroup, gtk_sys::GtkWindowGroupClass, WindowGroupClass>);

    match fn {
        get_type => || gtk_sys::gtk_window_group_get_type(),
    }
}

impl WindowGroup {
    pub fn new() -> WindowGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_window_group_new())
        }
    }
}

impl Default for WindowGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_WINDOW_GROUP: Option<&WindowGroup> = None;

pub trait WindowGroupExt: 'static {
    fn add_window<P: IsA<Window>>(&self, window: &P);

    //fn get_current_device_grab(&self, device: /*Ignored*/&gdk::Device) -> Option<Widget>;

    fn get_current_grab(&self) -> Option<Widget>;

    fn list_windows(&self) -> Vec<Window>;

    fn remove_window<P: IsA<Window>>(&self, window: &P);
}

impl<O: IsA<WindowGroup>> WindowGroupExt for O {
    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            gtk_sys::gtk_window_group_add_window(self.as_ref().to_glib_none().0, window.as_ref().to_glib_none().0);
        }
    }

    //fn get_current_device_grab(&self, device: /*Ignored*/&gdk::Device) -> Option<Widget> {
    //    unsafe { TODO: call gtk_sys:gtk_window_group_get_current_device_grab() }
    //}

    fn get_current_grab(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_window_group_get_current_grab(self.as_ref().to_glib_none().0))
        }
    }

    fn list_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_window_group_list_windows(self.as_ref().to_glib_none().0))
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            gtk_sys::gtk_window_group_remove_window(self.as_ref().to_glib_none().0, window.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for WindowGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WindowGroup")
    }
}
