// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use cairo_sys;
use gdk_sys;
use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use CairoContext;
use Cursor;
use Device;
use Display;
use Event;
use FrameClock;
use GLContext;
use ModifierType;
use Monitor;
use VulkanContext;

glib_wrapper! {
    pub struct Surface(Object<gdk_sys::GdkSurface, gdk_sys::GdkSurfaceClass, SurfaceClass>);

    match fn {
        get_type => || gdk_sys::gdk_surface_get_type(),
    }
}

impl Surface {
    pub fn new_popup(parent: &Surface, autohide: bool) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_new_popup(
                parent.to_glib_none().0,
                autohide.to_glib(),
            ))
        }
    }

    pub fn new_toplevel(display: &Display) -> Surface {
        skip_assert_initialized!();
        unsafe { from_glib_full(gdk_sys::gdk_surface_new_toplevel(display.to_glib_none().0)) }
    }

    pub fn beep(&self) {
        unsafe {
            gdk_sys::gdk_surface_beep(self.to_glib_none().0);
        }
    }

    pub fn create_cairo_context(&self) -> Option<CairoContext> {
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_create_cairo_context(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn create_gl_context(&self) -> Result<GLContext, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_sys::gdk_surface_create_gl_context(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn create_vulkan_context(&self) -> Result<VulkanContext, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_sys::gdk_surface_create_vulkan_context(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn destroy(&self) {
        unsafe {
            gdk_sys::gdk_surface_destroy(self.to_glib_none().0);
        }
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        unsafe { from_glib_none(gdk_sys::gdk_surface_get_cursor(self.to_glib_none().0)) }
    }

    pub fn get_device_cursor(&self, device: &Device) -> Option<Cursor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_surface_get_device_cursor(
                self.to_glib_none().0,
                device.to_glib_none().0,
            ))
        }
    }

    pub fn get_device_position(&self, device: &Device) -> Option<(f64, f64, ModifierType)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut mask = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_surface_get_device_position(
                self.to_glib_none().0,
                device.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                mask.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            let mask = mask.assume_init();
            if ret {
                Some((x, y, from_glib(mask)))
            } else {
                None
            }
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(gdk_sys::gdk_surface_get_display(self.to_glib_none().0)) }
    }

    pub fn get_frame_clock(&self) -> Option<FrameClock> {
        unsafe { from_glib_none(gdk_sys::gdk_surface_get_frame_clock(self.to_glib_none().0)) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { gdk_sys::gdk_surface_get_height(self.to_glib_none().0) }
    }

    pub fn get_mapped(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_surface_get_mapped(self.to_glib_none().0)) }
    }

    pub fn get_scale_factor(&self) -> i32 {
        unsafe { gdk_sys::gdk_surface_get_scale_factor(self.to_glib_none().0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { gdk_sys::gdk_surface_get_width(self.to_glib_none().0) }
    }

    pub fn hide(&self) {
        unsafe {
            gdk_sys::gdk_surface_hide(self.to_glib_none().0);
        }
    }

    pub fn is_destroyed(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_surface_is_destroyed(self.to_glib_none().0)) }
    }

    pub fn queue_render(&self) {
        unsafe {
            gdk_sys::gdk_surface_queue_render(self.to_glib_none().0);
        }
    }

    pub fn set_cursor(&self, cursor: Option<&Cursor>) {
        unsafe {
            gdk_sys::gdk_surface_set_cursor(self.to_glib_none().0, cursor.to_glib_none().0);
        }
    }

    pub fn set_device_cursor(&self, device: &Device, cursor: &Cursor) {
        unsafe {
            gdk_sys::gdk_surface_set_device_cursor(
                self.to_glib_none().0,
                device.to_glib_none().0,
                cursor.to_glib_none().0,
            );
        }
    }

    pub fn set_input_region(&self, region: &mut cairo::Region) {
        unsafe {
            gdk_sys::gdk_surface_set_input_region(
                self.to_glib_none().0,
                region.to_glib_none_mut().0,
            );
        }
    }

    pub fn set_opaque_region(&self, region: Option<&cairo::Region>) {
        unsafe {
            gdk_sys::gdk_surface_set_opaque_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }
    }

    pub fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            gdk_sys::gdk_surface_set_shadow_width(self.to_glib_none().0, left, right, top, bottom);
        }
    }

    pub fn connect_enter_monitor<F: Fn(&Surface, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_monitor_trampoline<F: Fn(&Surface, &Monitor) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            monitor: *mut gdk_sys::GdkMonitor,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter-monitor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_monitor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_event<F: Fn(&Surface, &Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<F: Fn(&Surface, &Event) -> bool + 'static>(
            this: *mut gdk_sys::GdkSurface,
            event: *mut gdk_sys::GdkEvent,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(event)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_leave_monitor<F: Fn(&Surface, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn leave_monitor_trampoline<F: Fn(&Surface, &Monitor) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            monitor: *mut gdk_sys::GdkMonitor,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave-monitor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_monitor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_render<F: Fn(&Surface, &cairo::Region) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<
            F: Fn(&Surface, &cairo::Region) -> bool + 'static,
        >(
            this: *mut gdk_sys::GdkSurface,
            region: *mut cairo_sys::cairo_region_t,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(region)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"render\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    render_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_size_changed<F: Fn(&Surface, i32, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn size_changed_trampoline<F: Fn(&Surface, i32, i32) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            width: libc::c_int,
            height: libc::c_int,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), width, height)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"size-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    size_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_cursor_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_cursor_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cursor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_height_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mapped_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mapped_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mapped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mapped_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_width_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut gdk_sys::GdkSurface,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Surface")
    }
}
