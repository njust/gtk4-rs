// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PageSetup;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use std::mem;

glib_wrapper! {
    pub struct PrintContext(Object<gtk_sys::GtkPrintContext, PrintContextClass>);

    match fn {
        get_type => || gtk_sys::gtk_print_context_get_type(),
    }
}

impl PrintContext {
    //pub fn create_pango_context(&self) -> /*Ignored*/Option<pango::Context> {
    //    unsafe { TODO: call gtk_sys:gtk_print_context_create_pango_context() }
    //}

    //pub fn create_pango_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call gtk_sys:gtk_print_context_create_pango_layout() }
    //}

    //pub fn get_cairo_context(&self) -> /*Ignored*/Option<cairo::Context> {
    //    unsafe { TODO: call gtk_sys:gtk_print_context_get_cairo_context() }
    //}

    pub fn get_dpi_x(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_print_context_get_dpi_x(self.to_glib_none().0)
        }
    }

    pub fn get_dpi_y(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_print_context_get_dpi_y(self.to_glib_none().0)
        }
    }

    pub fn get_hard_margins(&self) -> Option<(f64, f64, f64, f64)> {
        unsafe {
            let mut top = mem::uninitialized();
            let mut bottom = mem::uninitialized();
            let mut left = mem::uninitialized();
            let mut right = mem::uninitialized();
            let ret = from_glib(gtk_sys::gtk_print_context_get_hard_margins(self.to_glib_none().0, &mut top, &mut bottom, &mut left, &mut right));
            if ret { Some((top, bottom, left, right)) } else { None }
        }
    }

    pub fn get_height(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_print_context_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_page_setup(&self) -> Option<PageSetup> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_context_get_page_setup(self.to_glib_none().0))
        }
    }

    //pub fn get_pango_fontmap(&self) -> /*Ignored*/Option<pango::FontMap> {
    //    unsafe { TODO: call gtk_sys:gtk_print_context_get_pango_fontmap() }
    //}

    pub fn get_width(&self) -> f64 {
        unsafe {
            gtk_sys::gtk_print_context_get_width(self.to_glib_none().0)
        }
    }

    //pub fn set_cairo_context(&self, cr: /*Ignored*/&mut cairo::Context, dpi_x: f64, dpi_y: f64) {
    //    unsafe { TODO: call gtk_sys:gtk_print_context_set_cairo_context() }
    //}
}

impl fmt::Display for PrintContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrintContext")
    }
}
