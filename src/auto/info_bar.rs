// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use Button;
use Container;
use MessageType;
use Orientable;
use ResponseType;
use Widget;
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
    pub struct InfoBar(Object<gtk_sys::GtkInfoBar, gtk_sys::GtkInfoBarClass, InfoBarClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_info_bar_get_type(),
    }
}

impl InfoBar {
    pub fn new() -> InfoBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_info_bar_new()).unsafe_cast()
        }
    }

    //pub fn new_with_buttons(first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> InfoBar {
    //    unsafe { TODO: call gtk_sys:gtk_info_bar_new_with_buttons() }
    //}
}

impl Default for InfoBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_INFO_BAR: Option<&InfoBar> = None;

pub trait InfoBarExt: 'static {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType);

    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Option<Button>;

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_action_area(&self) -> Option<Widget>;

    fn get_content_area(&self) -> Option<Widget>;

    fn get_message_type(&self) -> MessageType;

    fn get_revealed(&self) -> bool;

    fn get_show_close_button(&self) -> bool;

    fn response(&self, response_id: ResponseType);

    fn set_default_response(&self, response_id: ResponseType);

    fn set_message_type(&self, message_type: MessageType);

    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool);

    fn set_revealed(&self, revealed: bool);

    fn set_show_close_button(&self, setting: bool);

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_close(&self);

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InfoBar>> InfoBarExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType) {
        unsafe {
            gtk_sys::gtk_info_bar_add_action_widget(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, response_id.to_glib());
        }
    }

    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Option<Button> {
        unsafe {
            from_glib_none(gtk_sys::gtk_info_bar_add_button(self.as_ref().to_glib_none().0, button_text.to_glib_none().0, response_id.to_glib()))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_info_bar_add_buttons() }
    //}

    fn get_action_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_info_bar_get_action_area(self.as_ref().to_glib_none().0))
        }
    }

    fn get_content_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_info_bar_get_content_area(self.as_ref().to_glib_none().0))
        }
    }

    fn get_message_type(&self) -> MessageType {
        unsafe {
            from_glib(gtk_sys::gtk_info_bar_get_message_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_revealed(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_info_bar_get_revealed(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_info_bar_get_show_close_button(self.as_ref().to_glib_none().0))
        }
    }

    fn response(&self, response_id: ResponseType) {
        unsafe {
            gtk_sys::gtk_info_bar_response(self.as_ref().to_glib_none().0, response_id.to_glib());
        }
    }

    fn set_default_response(&self, response_id: ResponseType) {
        unsafe {
            gtk_sys::gtk_info_bar_set_default_response(self.as_ref().to_glib_none().0, response_id.to_glib());
        }
    }

    fn set_message_type(&self, message_type: MessageType) {
        unsafe {
            gtk_sys::gtk_info_bar_set_message_type(self.as_ref().to_glib_none().0, message_type.to_glib());
        }
    }

    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool) {
        unsafe {
            gtk_sys::gtk_info_bar_set_response_sensitive(self.as_ref().to_glib_none().0, response_id.to_glib(), setting.to_glib());
        }
    }

    fn set_revealed(&self, revealed: bool) {
        unsafe {
            gtk_sys::gtk_info_bar_set_revealed(self.as_ref().to_glib_none().0, revealed.to_glib());
        }
    }

    fn set_show_close_button(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_info_bar_set_show_close_button(self.as_ref().to_glib_none().0, setting.to_glib());
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

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"response\0".as_ptr() as *const _,
                Some(transmute(response_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::message-type\0".as_ptr() as *const _,
                Some(transmute(notify_message_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::revealed\0".as_ptr() as *const _,
                Some(transmute(notify_revealed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute(notify_show_close_button_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn close_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkInfoBar, f: glib_sys::gpointer)
where P: IsA<InfoBar> {
    let f: &F = &*(f as *const F);
    f(&InfoBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn response_trampoline<P, F: Fn(&P, ResponseType) + 'static>(this: *mut gtk_sys::GtkInfoBar, response_id: gtk_sys::GtkResponseType, f: glib_sys::gpointer)
where P: IsA<InfoBar> {
    let f: &F = &*(f as *const F);
    f(&InfoBar::from_glib_borrow(this).unsafe_cast(), from_glib(response_id))
}

unsafe extern "C" fn notify_message_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkInfoBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<InfoBar> {
    let f: &F = &*(f as *const F);
    f(&InfoBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_revealed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkInfoBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<InfoBar> {
    let f: &F = &*(f as *const F);
    f(&InfoBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_close_button_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkInfoBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<InfoBar> {
    let f: &F = &*(f as *const F);
    f(&InfoBar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for InfoBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InfoBar")
    }
}
