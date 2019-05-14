// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::GString;
use glib::translate::*;
use gtk_sys;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecentInfo(Shared<gtk_sys::GtkRecentInfo>);

    match fn {
        ref => |ptr| gtk_sys::gtk_recent_info_ref(ptr),
        unref => |ptr| gtk_sys::gtk_recent_info_unref(ptr),
        get_type => || gtk_sys::gtk_recent_info_get_type(),
    }
}

impl RecentInfo {
    //pub fn create_app_info(&self, app_name: Option<&str>) -> Result</*Ignored*/Option<gio::AppInfo>, Error> {
    //    unsafe { TODO: call gtk_sys:gtk_recent_info_create_app_info() }
    //}

    pub fn exists(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_info_exists(self.to_glib_none().0))
        }
    }

    pub fn get_added(&self) -> libc::c_long {
        unsafe {
            gtk_sys::gtk_recent_info_get_added(self.to_glib_none().0)
        }
    }

    pub fn get_age(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_recent_info_get_age(self.to_glib_none().0)
        }
    }

    pub fn get_application_info(&self, app_name: &str) -> Option<(GString, u32, libc::c_long)> {
        unsafe {
            let mut app_exec = ptr::null();
            let mut count = mem::uninitialized();
            let mut time_ = mem::uninitialized();
            let ret = from_glib(gtk_sys::gtk_recent_info_get_application_info(self.to_glib_none().0, app_name.to_glib_none().0, &mut app_exec, &mut count, &mut time_));
            if ret { Some((from_glib_none(app_exec), count, time_)) } else { None }
        }
    }

    pub fn get_applications(&self) -> Vec<GString> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(gtk_sys::gtk_recent_info_get_applications(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    pub fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_recent_info_get_description(self.to_glib_none().0))
        }
    }

    pub fn get_display_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_recent_info_get_display_name(self.to_glib_none().0))
        }
    }

    //pub fn get_gicon(&self) -> /*Ignored*/Option<gio::Icon> {
    //    unsafe { TODO: call gtk_sys:gtk_recent_info_get_gicon() }
    //}

    pub fn get_groups(&self) -> Vec<GString> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(gtk_sys::gtk_recent_info_get_groups(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    pub fn get_mime_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_recent_info_get_mime_type(self.to_glib_none().0))
        }
    }

    pub fn get_modified(&self) -> libc::c_long {
        unsafe {
            gtk_sys::gtk_recent_info_get_modified(self.to_glib_none().0)
        }
    }

    pub fn get_private_hint(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_info_get_private_hint(self.to_glib_none().0))
        }
    }

    pub fn get_short_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_recent_info_get_short_name(self.to_glib_none().0))
        }
    }

    pub fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_recent_info_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_uri_display(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_recent_info_get_uri_display(self.to_glib_none().0))
        }
    }

    pub fn get_visited(&self) -> libc::c_long {
        unsafe {
            gtk_sys::gtk_recent_info_get_visited(self.to_glib_none().0)
        }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_info_has_application(self.to_glib_none().0, app_name.to_glib_none().0))
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_info_has_group(self.to_glib_none().0, group_name.to_glib_none().0))
        }
    }

    pub fn is_local(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_info_is_local(self.to_glib_none().0))
        }
    }

    pub fn last_application(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_recent_info_last_application(self.to_glib_none().0))
        }
    }

    pub fn match_(&self, info_b: &RecentInfo) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_info_match(self.to_glib_none().0, info_b.to_glib_none().0))
        }
    }
}
