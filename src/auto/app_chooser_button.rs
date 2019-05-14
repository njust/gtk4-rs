// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AppChooser;
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
    pub struct AppChooserButton(Object<gtk_sys::GtkAppChooserButton, gtk_sys::GtkAppChooserButtonClass, AppChooserButtonClass>) @extends Widget, @implements Buildable, AppChooser;

    match fn {
        get_type => || gtk_sys::gtk_app_chooser_button_get_type(),
    }
}

impl AppChooserButton {
    pub fn new(content_type: &str) -> AppChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_app_chooser_button_new(content_type.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_APP_CHOOSER_BUTTON: Option<&AppChooserButton> = None;

pub trait AppChooserButtonExt: 'static {
    //fn append_custom_item(&self, name: &str, label: &str, icon: /*Ignored*/&gio::Icon);

    fn append_separator(&self);

    fn get_heading(&self) -> Option<GString>;

    fn get_show_default_item(&self) -> bool;

    fn get_show_dialog_item(&self) -> bool;

    fn set_active_custom_item(&self, name: &str);

    fn set_heading(&self, heading: &str);

    fn set_show_default_item(&self, setting: bool);

    fn set_show_dialog_item(&self, setting: bool);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_custom_item_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_default_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_dialog_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserButton>> AppChooserButtonExt for O {
    //fn append_custom_item(&self, name: &str, label: &str, icon: /*Ignored*/&gio::Icon) {
    //    unsafe { TODO: call gtk_sys:gtk_app_chooser_button_append_custom_item() }
    //}

    fn append_separator(&self) {
        unsafe {
            gtk_sys::gtk_app_chooser_button_append_separator(self.as_ref().to_glib_none().0);
        }
    }

    fn get_heading(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_app_chooser_button_get_heading(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_default_item(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_app_chooser_button_get_show_default_item(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_dialog_item(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_app_chooser_button_get_show_dialog_item(self.as_ref().to_glib_none().0))
        }
    }

    fn set_active_custom_item(&self, name: &str) {
        unsafe {
            gtk_sys::gtk_app_chooser_button_set_active_custom_item(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_heading(&self, heading: &str) {
        unsafe {
            gtk_sys::gtk_app_chooser_button_set_heading(self.as_ref().to_glib_none().0, heading.to_glib_none().0);
        }
    }

    fn set_show_default_item(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_app_chooser_button_set_show_default_item(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn set_show_dialog_item(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_app_chooser_button_set_show_dialog_item(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_custom_item_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"custom-item-activated\0".as_ptr() as *const _,
                Some(transmute(custom_item_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::heading\0".as_ptr() as *const _,
                Some(transmute(notify_heading_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_default_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-default-item\0".as_ptr() as *const _,
                Some(transmute(notify_show_default_item_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_dialog_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-dialog-item\0".as_ptr() as *const _,
                Some(transmute(notify_show_dialog_item_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAppChooserButton, f: glib_sys::gpointer)
where P: IsA<AppChooserButton> {
    let f: &F = &*(f as *const F);
    f(&AppChooserButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn custom_item_activated_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut gtk_sys::GtkAppChooserButton, item_name: *mut libc::c_char, f: glib_sys::gpointer)
where P: IsA<AppChooserButton> {
    let f: &F = &*(f as *const F);
    f(&AppChooserButton::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(item_name))
}

unsafe extern "C" fn notify_heading_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAppChooserButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AppChooserButton> {
    let f: &F = &*(f as *const F);
    f(&AppChooserButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_default_item_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAppChooserButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AppChooserButton> {
    let f: &F = &*(f as *const F);
    f(&AppChooserButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_dialog_item_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAppChooserButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AppChooserButton> {
    let f: &F = &*(f as *const F);
    f(&AppChooserButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AppChooserButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppChooserButton")
    }
}
