// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Border;
use CssSection;
use StateFlags;
use StyleContextPrintFlags;
use StyleProvider;
use WidgetPath;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct StyleContext(Object<gtk_sys::GtkStyleContext, gtk_sys::GtkStyleContextClass, StyleContextClass>);

    match fn {
        get_type => || gtk_sys::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    pub fn new() -> StyleContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_style_context_new())
        }
    }

    //pub fn add_provider_for_display<P: IsA<StyleProvider>>(display: /*Ignored*/&gdk::Display, provider: &P, priority: u32) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_add_provider_for_display() }
    //}

    //pub fn remove_provider_for_display<P: IsA<StyleProvider>>(display: /*Ignored*/&gdk::Display, provider: &P) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_remove_provider_for_display() }
    //}

    //pub fn reset_widgets(display: /*Ignored*/&gdk::Display) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_reset_widgets() }
    //}
}

impl Default for StyleContext {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STYLE_CONTEXT: Option<&StyleContext> = None;

pub trait StyleContextExt: 'static {
    fn add_class(&self, class_name: &str);

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32);

    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_border(&self) -> Border;

    //fn get_color(&self, color: /*Ignored*/gdk::RGBA);

    //fn get_display(&self) -> /*Ignored*/Option<gdk::Display>;

    fn get_margin(&self) -> Border;

    fn get_padding(&self) -> Border;

    fn get_parent(&self) -> Option<StyleContext>;

    fn get_path(&self) -> Option<WidgetPath>;

    //fn get_property(&self, property: &str, value: /*Ignored*/glib::Value);

    fn get_scale(&self) -> i32;

    fn get_section(&self, property: &str) -> Option<CssSection>;

    fn get_state(&self) -> StateFlags;

    //fn get_valist(&self, first_property_name: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn has_class(&self, class_name: &str) -> bool;

    fn list_classes(&self) -> Vec<GString>;

    //fn lookup_color(&self, color_name: &str, color: /*Ignored*/gdk::RGBA) -> bool;

    fn remove_class(&self, class_name: &str);

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P);

    fn restore(&self);

    fn save(&self);

    //fn set_display(&self, display: /*Ignored*/&gdk::Display);

    fn set_parent<P: IsA<StyleContext>>(&self, parent: Option<&P>);

    fn set_path(&self, path: &WidgetPath);

    fn set_scale(&self, scale: i32);

    fn set_state(&self, flags: StateFlags);

    fn to_string(&self, flags: StyleContextPrintFlags) -> GString;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleContext>> StyleContextExt for O {
    fn add_class(&self, class_name: &str) {
        unsafe {
            gtk_sys::gtk_style_context_add_class(self.as_ref().to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32) {
        unsafe {
            gtk_sys::gtk_style_context_add_provider(self.as_ref().to_glib_none().0, provider.as_ref().to_glib_none().0, priority);
        }
    }

    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get() }
    //}

    fn get_border(&self) -> Border {
        unsafe {
            let mut border = Border::uninitialized();
            gtk_sys::gtk_style_context_get_border(self.as_ref().to_glib_none().0, border.to_glib_none_mut().0);
            border
        }
    }

    //fn get_color(&self, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_color() }
    //}

    //fn get_display(&self) -> /*Ignored*/Option<gdk::Display> {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_display() }
    //}

    fn get_margin(&self) -> Border {
        unsafe {
            let mut margin = Border::uninitialized();
            gtk_sys::gtk_style_context_get_margin(self.as_ref().to_glib_none().0, margin.to_glib_none_mut().0);
            margin
        }
    }

    fn get_padding(&self) -> Border {
        unsafe {
            let mut padding = Border::uninitialized();
            gtk_sys::gtk_style_context_get_padding(self.as_ref().to_glib_none().0, padding.to_glib_none_mut().0);
            padding
        }
    }

    fn get_parent(&self) -> Option<StyleContext> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_parent(self.as_ref().to_glib_none().0))
        }
    }

    fn get_path(&self) -> Option<WidgetPath> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_path(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_property(&self, property: &str, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_property() }
    //}

    fn get_scale(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_style_context_get_scale(self.as_ref().to_glib_none().0)
        }
    }

    fn get_section(&self, property: &str) -> Option<CssSection> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_section(self.as_ref().to_glib_none().0, property.to_glib_none().0))
        }
    }

    fn get_state(&self) -> StateFlags {
        unsafe {
            from_glib(gtk_sys::gtk_style_context_get_state(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_valist(&self, first_property_name: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_valist() }
    //}

    fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_style_context_has_class(self.as_ref().to_glib_none().0, class_name.to_glib_none().0))
        }
    }

    fn list_classes(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_style_context_list_classes(self.as_ref().to_glib_none().0))
        }
    }

    //fn lookup_color(&self, color_name: &str, color: /*Ignored*/gdk::RGBA) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_lookup_color() }
    //}

    fn remove_class(&self, class_name: &str) {
        unsafe {
            gtk_sys::gtk_style_context_remove_class(self.as_ref().to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P) {
        unsafe {
            gtk_sys::gtk_style_context_remove_provider(self.as_ref().to_glib_none().0, provider.as_ref().to_glib_none().0);
        }
    }

    fn restore(&self) {
        unsafe {
            gtk_sys::gtk_style_context_restore(self.as_ref().to_glib_none().0);
        }
    }

    fn save(&self) {
        unsafe {
            gtk_sys::gtk_style_context_save(self.as_ref().to_glib_none().0);
        }
    }

    //fn set_display(&self, display: /*Ignored*/&gdk::Display) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_set_display() }
    //}

    fn set_parent<P: IsA<StyleContext>>(&self, parent: Option<&P>) {
        unsafe {
            gtk_sys::gtk_style_context_set_parent(self.as_ref().to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_path(&self, path: &WidgetPath) {
        unsafe {
            gtk_sys::gtk_style_context_set_path(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn set_scale(&self, scale: i32) {
        unsafe {
            gtk_sys::gtk_style_context_set_scale(self.as_ref().to_glib_none().0, scale);
        }
    }

    fn set_state(&self, flags: StateFlags) {
        unsafe {
            gtk_sys::gtk_style_context_set_state(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    fn to_string(&self, flags: StyleContextPrintFlags) -> GString {
        unsafe {
            from_glib_full(gtk_sys::gtk_style_context_to_string(self.as_ref().to_glib_none().0, flags.to_glib()))
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::display\0".as_ptr() as *const _,
                Some(transmute(notify_display_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::parent\0".as_ptr() as *const _,
                Some(transmute(notify_parent_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStyleContext, f: glib_sys::gpointer)
where P: IsA<StyleContext> {
    let f: &F = &*(f as *const F);
    f(&StyleContext::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_display_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStyleContext, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StyleContext> {
    let f: &F = &*(f as *const F);
    f(&StyleContext::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStyleContext, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<StyleContext> {
    let f: &F = &*(f as *const F);
    f(&StyleContext::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for StyleContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleContext")
    }
}
