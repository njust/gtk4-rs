// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::GString;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CssSection(Shared<gtk_sys::GtkCssSection>);

    match fn {
        ref => |ptr| gtk_sys::gtk_css_section_ref(ptr),
        unref => |ptr| gtk_sys::gtk_css_section_unref(ptr),
        get_type => || gtk_sys::gtk_css_section_get_type(),
    }
}

impl CssSection {
    //pub fn new(file: /*Ignored*/Option<&gio::File>, start: /*Ignored*/&CssLocation, end: /*Ignored*/&CssLocation) -> CssSection {
    //    unsafe { TODO: call gtk_sys:gtk_css_section_new() }
    //}

    //pub fn get_end_location(&self) -> /*Ignored*/Option<CssLocation> {
    //    unsafe { TODO: call gtk_sys:gtk_css_section_get_end_location() }
    //}

    //pub fn get_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call gtk_sys:gtk_css_section_get_file() }
    //}

    pub fn get_parent(&self) -> Option<CssSection> {
        unsafe {
            from_glib_none(gtk_sys::gtk_css_section_get_parent(self.to_glib_none().0))
        }
    }

    //pub fn get_start_location(&self) -> /*Ignored*/Option<CssLocation> {
    //    unsafe { TODO: call gtk_sys:gtk_css_section_get_start_location() }
    //}

    //pub fn print(&self, string: /*Ignored*/&mut glib::String) {
    //    unsafe { TODO: call gtk_sys:gtk_css_section_print() }
    //}

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gtk_sys::gtk_css_section_to_string(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for CssSection {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
