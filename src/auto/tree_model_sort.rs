// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TreeDragSource;
use TreeIter;
use TreeModel;
use TreePath;
use TreeSortable;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct TreeModelSort(Object<gtk_sys::GtkTreeModelSort, gtk_sys::GtkTreeModelSortClass, TreeModelSortClass>) @implements TreeDragSource, TreeModel, TreeSortable;

    match fn {
        get_type => || gtk_sys::gtk_tree_model_sort_get_type(),
    }
}

pub const NONE_TREE_MODEL_SORT: Option<&TreeModelSort> = None;

pub trait TreeModelSortExt: 'static {
    fn clear_cache(&self);

    fn convert_child_iter_to_iter(&self, child_iter: &mut TreeIter) -> Option<TreeIter>;

    fn convert_child_path_to_path(&self, child_path: &mut TreePath) -> Option<TreePath>;

    fn convert_iter_to_child_iter(&self, sorted_iter: &mut TreeIter) -> TreeIter;

    fn convert_path_to_child_path(&self, sorted_path: &mut TreePath) -> Option<TreePath>;

    fn get_model(&self) -> Option<TreeModel>;

    fn iter_is_valid(&self, iter: &mut TreeIter) -> bool;

    fn reset_default_sort_func(&self);
}

impl<O: IsA<TreeModelSort>> TreeModelSortExt for O {
    fn clear_cache(&self) {
        unsafe {
            gtk_sys::gtk_tree_model_sort_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    fn convert_child_iter_to_iter(&self, child_iter: &mut TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut sort_iter = TreeIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_tree_model_sort_convert_child_iter_to_iter(self.as_ref().to_glib_none().0, sort_iter.to_glib_none_mut().0, child_iter.to_glib_none_mut().0));
            if ret { Some(sort_iter) } else { None }
        }
    }

    fn convert_child_path_to_path(&self, child_path: &mut TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_model_sort_convert_child_path_to_path(self.as_ref().to_glib_none().0, child_path.to_glib_none_mut().0))
        }
    }

    fn convert_iter_to_child_iter(&self, sorted_iter: &mut TreeIter) -> TreeIter {
        unsafe {
            let mut child_iter = TreeIter::uninitialized();
            gtk_sys::gtk_tree_model_sort_convert_iter_to_child_iter(self.as_ref().to_glib_none().0, child_iter.to_glib_none_mut().0, sorted_iter.to_glib_none_mut().0);
            child_iter
        }
    }

    fn convert_path_to_child_path(&self, sorted_path: &mut TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_model_sort_convert_path_to_child_path(self.as_ref().to_glib_none().0, sorted_path.to_glib_none_mut().0))
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tree_model_sort_get_model(self.as_ref().to_glib_none().0))
        }
    }

    fn iter_is_valid(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_model_sort_iter_is_valid(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn reset_default_sort_func(&self) {
        unsafe {
            gtk_sys::gtk_tree_model_sort_reset_default_sort_func(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for TreeModelSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeModelSort")
    }
}
