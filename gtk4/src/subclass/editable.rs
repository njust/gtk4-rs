// Take a look at the license at the top of the repository in the LICENSE file.

use super::widget::WidgetImpl;
use crate::Editable;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};
use libc::{c_char, c_int};

pub trait EditableImpl: WidgetImpl {
    fn insert_text(&self, editable: &Self::Type, text: &str, length: i32, position: &mut i32) {
        self.parent_insert_text(editable, text, length, position);
    }

    fn delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        self.parent_delete_text(editable, start_position, end_position)
    }

    fn changed(&self, editable: &Self::Type) {
        self.parent_changed(editable)
    }

    fn get_text(&self, editable: &Self::Type) -> GString;

    fn do_insert_text(&self, editable: &Self::Type, text: &str, length: i32, position: &mut i32) {
        self.parent_do_insert_text(editable, text, length, position)
    }

    fn do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        self.parent_do_delete_text(editable, start_position, end_position)
    }

    fn get_selection_bounds(&self, editable: &Self::Type) -> Option<(i32, i32)> {
        self.parent_get_selection_bounds(editable)
    }

    fn set_selection_bounds(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        self.parent_set_selection_bounds(editable, start_position, end_position)
    }
}

pub trait EditableImplExt: ObjectSubclass {
    fn parent_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    );
    fn parent_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32);
    fn parent_changed(&self, editable: &Self::Type);
    fn parent_do_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    );
    fn parent_do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32);
    fn parent_get_selection_bounds(&self, editable: &Self::Type) -> Option<(i32, i32)>;
    fn parent_set_selection_bounds(
        &self,
        editable: &Self::Type,
        start_position: i32,
        end_position: i32,
    );
    fn parent_get_text(&self, editable: &Self::Type) -> GString;
}

impl<T: EditableImpl> EditableImplExt for T {
    fn parent_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).insert_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    text.to_glib_none().0,
                    length,
                    position,
                );
            }
        }
    }

    fn parent_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).delete_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position,
                    end_position,
                );
            }
        }
    }

    fn parent_get_text(&self, editable: &Self::Type) -> GString {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;
            let func = (*parent_iface)
                .get_text
                .expect("no parent \"get_text\" implementation");

            from_glib_none(func(
                editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
            ))
        }
    }

    fn parent_changed(&self, editable: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).changed {
                func(editable.unsafe_cast_ref::<Editable>().to_glib_none().0);
            }
        }
    }

    fn parent_do_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).do_insert_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    text.to_glib_none().0,
                    length,
                    position,
                );
            }
        }
    }

    fn parent_do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).do_delete_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position,
                    end_position,
                );
            }
        }
    }

    fn parent_get_selection_bounds(&self, editable: &Self::Type) -> Option<(i32, i32)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).get_selection_bounds {
                let mut start_position = std::mem::MaybeUninit::uninit();
                let mut end_position = std::mem::MaybeUninit::uninit();
                if from_glib(func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position.as_mut_ptr(),
                    end_position.as_mut_ptr(),
                )) {
                    return Some((start_position.assume_init(), end_position.assume_init()));
                }
            }
            None
        }
    }

    fn parent_set_selection_bounds(
        &self,
        editable: &Self::Type,
        start_position: i32,
        end_position: i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).set_selection_bounds {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position,
                    end_position,
                );
            }
        }
    }
}

unsafe impl<T: EditableImpl> IsImplementable<T> for Editable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.insert_text = Some(editable_insert_text::<T>);
        iface.delete_text = Some(editable_delete_text::<T>);
        iface.changed = Some(editable_changed::<T>);
        iface.get_text = Some(editable_get_text::<T>);
        iface.do_insert_text = Some(editable_do_insert_text::<T>);
        iface.do_delete_text = Some(editable_do_delete_text::<T>);
        iface.get_selection_bounds = Some(editable_get_selection_bounds::<T>);
        iface.set_selection_bounds = Some(editable_set_selection_bounds::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn editable_insert_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    text: *const c_char,
    length: c_int,
    position: *mut c_int,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.insert_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &GString::from_glib_borrow(text),
        length,
        &mut *position,
    )
}

unsafe extern "C" fn editable_delete_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: c_int,
    end_position: c_int,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.delete_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}

unsafe extern "C" fn editable_changed<T: EditableImpl>(editable: *mut ffi::GtkEditable) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.changed(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
}

unsafe extern "C" fn editable_get_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
) -> *const c_char {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_text(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn editable_do_insert_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    text: *const c_char,
    length: i32,
    position: *mut i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.do_insert_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &GString::from_glib_borrow(text),
        length,
        &mut *position,
    )
}

unsafe extern "C" fn editable_do_delete_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: i32,
    end_position: i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.do_delete_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}

unsafe extern "C" fn editable_get_selection_bounds<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: *mut i32,
    end_position: *mut i32,
) -> glib::ffi::gboolean {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    if let Some((start_pos, end_pos)) =
        imp.get_selection_bounds(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
    {
        if !start_position.is_null() {
            *start_position = start_pos;
        }

        if !end_position.is_null() {
            *end_position = end_pos;
        }
        true.to_glib()
    } else {
        *start_position = 0;
        *end_position = 0;
        false.to_glib()
    }
}

unsafe extern "C" fn editable_set_selection_bounds<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: i32,
    end_position: i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.set_selection_bounds(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}
