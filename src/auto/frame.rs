// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use ShadowType;
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
    pub struct Frame(Object<gtk_sys::GtkFrame, gtk_sys::GtkFrameClass, FrameClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_frame_get_type(),
    }
}

impl Frame {
    pub fn new(label: Option<&str>) -> Frame {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_frame_new(label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_FRAME: Option<&Frame> = None;

pub trait FrameExt: 'static {
    fn get_label(&self) -> Option<GString>;

    fn get_label_align(&self) -> f32;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_shadow_type(&self) -> ShadowType;

    fn set_label(&self, label: Option<&str>);

    fn set_label_align(&self, xalign: f32);

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>);

    fn set_shadow_type(&self, type_: ShadowType);

    fn get_property_label_xalign(&self) -> f32;

    fn set_property_label_xalign(&self, label_xalign: f32);

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Frame>> FrameExt for O {
    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_frame_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_label_align(&self) -> f32 {
        unsafe {
            gtk_sys::gtk_frame_get_label_align(self.as_ref().to_glib_none().0)
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_frame_get_label_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(gtk_sys::gtk_frame_get_shadow_type(self.as_ref().to_glib_none().0))
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            gtk_sys::gtk_frame_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_align(&self, xalign: f32) {
        unsafe {
            gtk_sys::gtk_frame_set_label_align(self.as_ref().to_glib_none().0, xalign);
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_frame_set_label_widget(self.as_ref().to_glib_none().0, label_widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            gtk_sys::gtk_frame_set_shadow_type(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn get_property_label_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"label-xalign\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_label_xalign(&self, label_xalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"label-xalign\0".as_ptr() as *const _, Value::from(&label_xalign).to_glib_none().0);
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute(notify_label_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-xalign\0".as_ptr() as *const _,
                Some(transmute(notify_label_xalign_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shadow-type\0".as_ptr() as *const _,
                Some(transmute(notify_shadow_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFrame, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Frame> {
    let f: &F = &*(f as *const F);
    f(&Frame::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFrame, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Frame> {
    let f: &F = &*(f as *const F);
    f(&Frame::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_xalign_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFrame, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Frame> {
    let f: &F = &*(f as *const F);
    f(&Frame::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_shadow_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFrame, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Frame> {
    let f: &F = &*(f as *const F);
    f(&Frame::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frame")
    }
}
