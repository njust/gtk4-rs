// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Editable;
use EntryBuffer;
use InputHints;
use InputPurpose;
use Widget;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType;
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
    pub struct Text(Object<gtk_sys::GtkText, TextClass>) @extends Widget, @implements Buildable, Editable;

    match fn {
        get_type => || gtk_sys::gtk_text_get_type(),
    }
}

impl Text {
    pub fn new() -> Text {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_text_new()).unsafe_cast()
        }
    }

    pub fn new_with_buffer<P: IsA<EntryBuffer>>(buffer: &P) -> Text {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_text_new_with_buffer(buffer.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn get_activates_default(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_get_activates_default(self.to_glib_none().0))
        }
    }

    //pub fn get_attributes(&self) -> /*Ignored*/Option<pango::AttrList> {
    //    unsafe { TODO: call gtk_sys:gtk_text_get_attributes() }
    //}

    pub fn get_buffer(&self) -> Option<EntryBuffer> {
        unsafe {
            from_glib_none(gtk_sys::gtk_text_get_buffer(self.to_glib_none().0))
        }
    }

    pub fn get_input_hints(&self) -> InputHints {
        unsafe {
            from_glib(gtk_sys::gtk_text_get_input_hints(self.to_glib_none().0))
        }
    }

    pub fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            from_glib(gtk_sys::gtk_text_get_input_purpose(self.to_glib_none().0))
        }
    }

    pub fn get_invisible_char(&self) -> char {
        unsafe {
            from_glib(gtk_sys::gtk_text_get_invisible_char(self.to_glib_none().0))
        }
    }

    pub fn get_max_length(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_get_max_length(self.to_glib_none().0)
        }
    }

    pub fn get_overwrite_mode(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_get_overwrite_mode(self.to_glib_none().0))
        }
    }

    pub fn get_placeholder_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_text_get_placeholder_text(self.to_glib_none().0))
        }
    }

    //pub fn get_tabs(&self) -> /*Ignored*/Option<pango::TabArray> {
    //    unsafe { TODO: call gtk_sys:gtk_text_get_tabs() }
    //}

    pub fn get_text_length(&self) -> u16 {
        unsafe {
            gtk_sys::gtk_text_get_text_length(self.to_glib_none().0)
        }
    }

    pub fn get_visibility(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_get_visibility(self.to_glib_none().0))
        }
    }

    pub fn grab_focus_without_selecting(&self) {
        unsafe {
            gtk_sys::gtk_text_grab_focus_without_selecting(self.to_glib_none().0);
        }
    }

    pub fn set_activates_default(&self, activates: bool) {
        unsafe {
            gtk_sys::gtk_text_set_activates_default(self.to_glib_none().0, activates.to_glib());
        }
    }

    //pub fn set_attributes(&self, attrs: /*Ignored*/&pango::AttrList) {
    //    unsafe { TODO: call gtk_sys:gtk_text_set_attributes() }
    //}

    pub fn set_buffer<P: IsA<EntryBuffer>>(&self, buffer: &P) {
        unsafe {
            gtk_sys::gtk_text_set_buffer(self.to_glib_none().0, buffer.as_ref().to_glib_none().0);
        }
    }

    pub fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            gtk_sys::gtk_text_set_input_hints(self.to_glib_none().0, hints.to_glib());
        }
    }

    pub fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            gtk_sys::gtk_text_set_input_purpose(self.to_glib_none().0, purpose.to_glib());
        }
    }

    pub fn set_invisible_char(&self, ch: char) {
        unsafe {
            gtk_sys::gtk_text_set_invisible_char(self.to_glib_none().0, ch.to_glib());
        }
    }

    pub fn set_max_length(&self, length: i32) {
        unsafe {
            gtk_sys::gtk_text_set_max_length(self.to_glib_none().0, length);
        }
    }

    pub fn set_overwrite_mode(&self, overwrite: bool) {
        unsafe {
            gtk_sys::gtk_text_set_overwrite_mode(self.to_glib_none().0, overwrite.to_glib());
        }
    }

    pub fn set_placeholder_text(&self, text: Option<&str>) {
        unsafe {
            gtk_sys::gtk_text_set_placeholder_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    //pub fn set_tabs(&self, tabs: /*Ignored*/Option<&mut pango::TabArray>) {
    //    unsafe { TODO: call gtk_sys:gtk_text_set_tabs() }
    //}

    pub fn set_visibility(&self, visible: bool) {
        unsafe {
            gtk_sys::gtk_text_set_visibility(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn unset_invisible_char(&self) {
        unsafe {
            gtk_sys::gtk_text_unset_invisible_char(self.to_glib_none().0);
        }
    }

    pub fn get_property_enable_emoji_completion(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"enable-emoji-completion\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_enable_emoji_completion(&self, enable_emoji_completion: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"enable-emoji-completion\0".as_ptr() as *const _, Value::from(&enable_emoji_completion).to_glib_none().0);
        }
    }

    pub fn get_property_im_module(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"im-module\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_im_module(&self, im_module: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"im-module\0".as_ptr() as *const _, Value::from(im_module).to_glib_none().0);
        }
    }

    pub fn get_property_invisible_char_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"invisible-char-set\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_invisible_char_set(&self, invisible_char_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"invisible-char-set\0".as_ptr() as *const _, Value::from(&invisible_char_set).to_glib_none().0);
        }
    }

    pub fn get_property_populate_all(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"populate-all\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_populate_all(&self, populate_all: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"populate-all\0".as_ptr() as *const _, Value::from(&populate_all).to_glib_none().0);
        }
    }

    pub fn get_property_propagate_text_width(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"propagate-text-width\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_propagate_text_width(&self, propagate_text_width: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"propagate-text-width\0".as_ptr() as *const _, Value::from(&propagate_text_width).to_glib_none().0);
        }
    }

    pub fn get_property_scroll_offset(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"scroll-offset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_truncate_multiline(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"truncate-multiline\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_truncate_multiline(&self, truncate_multiline: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"truncate-multiline\0".as_ptr() as *const _, Value::from(&truncate_multiline).to_glib_none().0);
        }
    }

    pub fn connect_property_activates_default_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::activates-default\0".as_ptr() as *const _,
                Some(transmute(notify_activates_default_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_attributes_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attributes\0".as_ptr() as *const _,
                Some(transmute(notify_attributes_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_buffer_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::buffer\0".as_ptr() as *const _,
                Some(transmute(notify_buffer_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_enable_emoji_completion_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enable-emoji-completion\0".as_ptr() as *const _,
                Some(transmute(notify_enable_emoji_completion_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_im_module_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::im-module\0".as_ptr() as *const _,
                Some(transmute(notify_im_module_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_input_hints_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::input-hints\0".as_ptr() as *const _,
                Some(transmute(notify_input_hints_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_input_purpose_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::input-purpose\0".as_ptr() as *const _,
                Some(transmute(notify_input_purpose_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_invisible_char_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::invisible-char\0".as_ptr() as *const _,
                Some(transmute(notify_invisible_char_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_invisible_char_set_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::invisible-char-set\0".as_ptr() as *const _,
                Some(transmute(notify_invisible_char_set_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_max_length_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-length\0".as_ptr() as *const _,
                Some(transmute(notify_max_length_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_overwrite_mode_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::overwrite-mode\0".as_ptr() as *const _,
                Some(transmute(notify_overwrite_mode_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_placeholder_text_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::placeholder-text\0".as_ptr() as *const _,
                Some(transmute(notify_placeholder_text_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_populate_all_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::populate-all\0".as_ptr() as *const _,
                Some(transmute(notify_populate_all_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_propagate_text_width_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::propagate-text-width\0".as_ptr() as *const _,
                Some(transmute(notify_propagate_text_width_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_scroll_offset_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::scroll-offset\0".as_ptr() as *const _,
                Some(transmute(notify_scroll_offset_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_tabs_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tabs\0".as_ptr() as *const _,
                Some(transmute(notify_tabs_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_truncate_multiline_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::truncate-multiline\0".as_ptr() as *const _,
                Some(transmute(notify_truncate_multiline_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_visibility_notify<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visibility\0".as_ptr() as *const _,
                Some(transmute(notify_visibility_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

unsafe extern "C" fn notify_activates_default_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_attributes_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_buffer_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_enable_emoji_completion_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_im_module_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_input_hints_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_input_purpose_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_invisible_char_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_invisible_char_set_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_max_length_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_overwrite_mode_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_placeholder_text_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_populate_all_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_propagate_text_width_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_scroll_offset_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_tabs_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_truncate_multiline_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_visibility_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text")
    }
}
