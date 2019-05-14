// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use FileChooser;
use FileChooserAction;
use NativeDialog;
use Window;
use glib::GString;
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
    pub struct FileChooserNative(Object<gtk_sys::GtkFileChooserNative, gtk_sys::GtkFileChooserNativeClass, FileChooserNativeClass>) @extends NativeDialog, @implements FileChooser;

    match fn {
        get_type => || gtk_sys::gtk_file_chooser_native_get_type(),
    }
}

impl FileChooserNative {
    pub fn new<P: IsA<Window>>(title: Option<&str>, parent: Option<&P>, action: FileChooserAction, accept_label: Option<&str>, cancel_label: Option<&str>) -> FileChooserNative {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_native_new(title.to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0, action.to_glib(), accept_label.to_glib_none().0, cancel_label.to_glib_none().0))
        }
    }
}

pub const NONE_FILE_CHOOSER_NATIVE: Option<&FileChooserNative> = None;

pub trait FileChooserNativeExt: 'static {
    fn get_accept_label(&self) -> Option<GString>;

    fn get_cancel_label(&self) -> Option<GString>;

    fn set_accept_label(&self, accept_label: Option<&str>);

    fn set_cancel_label(&self, cancel_label: Option<&str>);

    fn connect_property_accept_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cancel_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserNative>> FileChooserNativeExt for O {
    fn get_accept_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_chooser_native_get_accept_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cancel_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_chooser_native_get_cancel_label(self.as_ref().to_glib_none().0))
        }
    }

    fn set_accept_label(&self, accept_label: Option<&str>) {
        unsafe {
            gtk_sys::gtk_file_chooser_native_set_accept_label(self.as_ref().to_glib_none().0, accept_label.to_glib_none().0);
        }
    }

    fn set_cancel_label(&self, cancel_label: Option<&str>) {
        unsafe {
            gtk_sys::gtk_file_chooser_native_set_cancel_label(self.as_ref().to_glib_none().0, cancel_label.to_glib_none().0);
        }
    }

    fn connect_property_accept_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accept-label\0".as_ptr() as *const _,
                Some(transmute(notify_accept_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_cancel_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cancel-label\0".as_ptr() as *const _,
                Some(transmute(notify_cancel_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_accept_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFileChooserNative, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<FileChooserNative> {
    let f: &F = &*(f as *const F);
    f(&FileChooserNative::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_cancel_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFileChooserNative, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<FileChooserNative> {
    let f: &F = &*(f as *const F);
    f(&FileChooserNative::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FileChooserNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileChooserNative")
    }
}
