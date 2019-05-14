// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellLayout;
use CellRenderer;
use Container;
use IconViewDropPosition;
use MovementStep;
use Orientation;
use Scrollable;
use SelectionMode;
use Tooltip;
use TreeIter;
use TreeModel;
use TreePath;
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
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct IconView(Object<gtk_sys::GtkIconView, gtk_sys::GtkIconViewClass, IconViewClass>) @extends Container, Widget, @implements Buildable, CellLayout, Scrollable;

    match fn {
        get_type => || gtk_sys::gtk_icon_view_get_type(),
    }
}

impl IconView {
    pub fn new() -> IconView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_icon_view_new()).unsafe_cast()
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> IconView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_icon_view_new_with_area(area.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_model<P: IsA<TreeModel>>(model: &P) -> IconView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_icon_view_new_with_model(model.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for IconView {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ICON_VIEW: Option<&IconView> = None;

pub trait IconViewExt: 'static {
    //fn create_drag_icon(&self, path: &TreePath) -> /*Ignored*/Option<gdk::Paintable>;

    //fn enable_model_drag_dest(&self, formats: /*Ignored*/&gdk::ContentFormats, actions: /*Ignored*/gdk::DragAction);

    //fn enable_model_drag_source(&self, start_button_mask: /*Ignored*/gdk::ModifierType, formats: /*Ignored*/&gdk::ContentFormats, actions: /*Ignored*/gdk::DragAction);

    fn get_activate_on_single_click(&self) -> bool;

    //fn get_cell_rect<P: IsA<CellRenderer>>(&self, path: &TreePath, cell: Option<&P>, rect: /*Ignored*/gdk::Rectangle) -> bool;

    fn get_column_spacing(&self) -> i32;

    fn get_columns(&self) -> i32;

    fn get_cursor(&self) -> Option<(TreePath, CellRenderer)>;

    fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32) -> Option<(TreePath, IconViewDropPosition)>;

    fn get_drag_dest_item(&self) -> (TreePath, IconViewDropPosition);

    fn get_item_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, CellRenderer)>;

    fn get_item_column(&self, path: &TreePath) -> i32;

    fn get_item_orientation(&self) -> Orientation;

    fn get_item_padding(&self) -> i32;

    fn get_item_row(&self, path: &TreePath) -> i32;

    fn get_item_width(&self) -> i32;

    fn get_margin(&self) -> i32;

    fn get_markup_column(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath>;

    fn get_pixbuf_column(&self) -> i32;

    fn get_reorderable(&self) -> bool;

    fn get_row_spacing(&self) -> i32;

    fn get_selected_items(&self) -> Vec<TreePath>;

    fn get_selection_mode(&self) -> SelectionMode;

    fn get_spacing(&self) -> i32;

    fn get_text_column(&self) -> i32;

    fn get_tooltip_column(&self) -> i32;

    fn get_tooltip_context(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool) -> Option<(TreeModel, TreePath, TreeIter)>;

    fn get_visible_range(&self) -> Option<(TreePath, TreePath)>;

    fn item_activated(&self, path: &TreePath);

    fn path_is_selected(&self, path: &TreePath) -> bool;

    fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32);

    fn select_all(&self);

    fn select_path(&self, path: &TreePath);

    fn selected_foreach<P: FnMut(&IconView, &TreePath)>(&self, func: P);

    fn set_activate_on_single_click(&self, single: bool);

    fn set_column_spacing(&self, column_spacing: i32);

    fn set_columns(&self, columns: i32);

    fn set_cursor<P: IsA<CellRenderer>>(&self, path: &TreePath, cell: Option<&P>, start_editing: bool);

    fn set_drag_dest_item(&self, path: Option<&TreePath>, pos: IconViewDropPosition);

    fn set_item_orientation(&self, orientation: Orientation);

    fn set_item_padding(&self, item_padding: i32);

    fn set_item_width(&self, item_width: i32);

    fn set_margin(&self, margin: i32);

    fn set_markup_column(&self, column: i32);

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>);

    fn set_pixbuf_column(&self, column: i32);

    fn set_reorderable(&self, reorderable: bool);

    fn set_row_spacing(&self, row_spacing: i32);

    fn set_selection_mode(&self, mode: SelectionMode);

    fn set_spacing(&self, spacing: i32);

    fn set_text_column(&self, column: i32);

    fn set_tooltip_cell<P: IsA<CellRenderer>>(&self, tooltip: &Tooltip, path: &TreePath, cell: Option<&P>);

    fn set_tooltip_column(&self, column: i32);

    fn set_tooltip_item(&self, tooltip: &Tooltip, path: &TreePath);

    fn unselect_all(&self);

    fn unselect_path(&self, path: &TreePath);

    fn unset_model_drag_dest(&self);

    fn unset_model_drag_source(&self);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn connect_activate_cursor_item<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_cursor_item(&self) -> bool;

    fn connect_item_activated<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_cursor(&self, step: MovementStep, count: i32) -> bool;

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_all(&self);

    fn connect_select_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_cursor_item(&self);

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toggle_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_cursor_item(&self);

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_unselect_all(&self);

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_columns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_markup_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IconView>> IconViewExt for O {
    //fn create_drag_icon(&self, path: &TreePath) -> /*Ignored*/Option<gdk::Paintable> {
    //    unsafe { TODO: call gtk_sys:gtk_icon_view_create_drag_icon() }
    //}

    //fn enable_model_drag_dest(&self, formats: /*Ignored*/&gdk::ContentFormats, actions: /*Ignored*/gdk::DragAction) {
    //    unsafe { TODO: call gtk_sys:gtk_icon_view_enable_model_drag_dest() }
    //}

    //fn enable_model_drag_source(&self, start_button_mask: /*Ignored*/gdk::ModifierType, formats: /*Ignored*/&gdk::ContentFormats, actions: /*Ignored*/gdk::DragAction) {
    //    unsafe { TODO: call gtk_sys:gtk_icon_view_enable_model_drag_source() }
    //}

    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_view_get_activate_on_single_click(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_cell_rect<P: IsA<CellRenderer>>(&self, path: &TreePath, cell: Option<&P>, rect: /*Ignored*/gdk::Rectangle) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_icon_view_get_cell_rect() }
    //}

    fn get_column_spacing(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_column_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn get_columns(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_columns(self.as_ref().to_glib_none().0)
        }
    }

    fn get_cursor(&self) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ret = from_glib(gtk_sys::gtk_icon_view_get_cursor(self.as_ref().to_glib_none().0, &mut path, &mut cell));
            if ret { Some((from_glib_full(path), from_glib_none(cell))) } else { None }
        }
    }

    fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32) -> Option<(TreePath, IconViewDropPosition)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            let ret = from_glib(gtk_sys::gtk_icon_view_get_dest_item_at_pos(self.as_ref().to_glib_none().0, drag_x, drag_y, &mut path, &mut pos));
            if ret { Some((from_glib_full(path), from_glib(pos))) } else { None }
        }
    }

    fn get_drag_dest_item(&self) -> (TreePath, IconViewDropPosition) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            gtk_sys::gtk_icon_view_get_drag_dest_item(self.as_ref().to_glib_none().0, &mut path, &mut pos);
            (from_glib_full(path), from_glib(pos))
        }
    }

    fn get_item_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ret = from_glib(gtk_sys::gtk_icon_view_get_item_at_pos(self.as_ref().to_glib_none().0, x, y, &mut path, &mut cell));
            if ret { Some((from_glib_full(path), from_glib_none(cell))) } else { None }
        }
    }

    fn get_item_column(&self, path: &TreePath) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_item_column(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0))
        }
    }

    fn get_item_orientation(&self) -> Orientation {
        unsafe {
            from_glib(gtk_sys::gtk_icon_view_get_item_orientation(self.as_ref().to_glib_none().0))
        }
    }

    fn get_item_padding(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_item_padding(self.as_ref().to_glib_none().0)
        }
    }

    fn get_item_row(&self, path: &TreePath) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_item_row(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0))
        }
    }

    fn get_item_width(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_item_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_margin(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_margin(self.as_ref().to_glib_none().0)
        }
    }

    fn get_markup_column(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_markup_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_icon_view_get_model(self.as_ref().to_glib_none().0))
        }
    }

    fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_view_get_path_at_pos(self.as_ref().to_glib_none().0, x, y))
        }
    }

    fn get_pixbuf_column(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_pixbuf_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_view_get_reorderable(self.as_ref().to_glib_none().0))
        }
    }

    fn get_row_spacing(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_row_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn get_selected_items(&self) -> Vec<TreePath> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_icon_view_get_selected_items(self.as_ref().to_glib_none().0))
        }
    }

    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(gtk_sys::gtk_icon_view_get_selection_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn get_text_column(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_text_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_tooltip_column(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_view_get_tooltip_column(self.as_ref().to_glib_none().0)
        }
    }

    fn get_tooltip_context(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool) -> Option<(TreeModel, TreePath, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut path = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_icon_view_get_tooltip_context(self.as_ref().to_glib_none().0, x, y, keyboard_tip.to_glib(), &mut model, &mut path, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), from_glib_full(path), iter)) } else { None }
        }
    }

    fn get_visible_range(&self) -> Option<(TreePath, TreePath)> {
        unsafe {
            let mut start_path = ptr::null_mut();
            let mut end_path = ptr::null_mut();
            let ret = from_glib(gtk_sys::gtk_icon_view_get_visible_range(self.as_ref().to_glib_none().0, &mut start_path, &mut end_path));
            if ret { Some((from_glib_full(start_path), from_glib_full(end_path))) } else { None }
        }
    }

    fn item_activated(&self, path: &TreePath) {
        unsafe {
            gtk_sys::gtk_icon_view_item_activated(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_view_path_is_selected(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32) {
        unsafe {
            gtk_sys::gtk_icon_view_scroll_to_path(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0), use_align.to_glib(), row_align, col_align);
        }
    }

    fn select_all(&self) {
        unsafe {
            gtk_sys::gtk_icon_view_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_path(&self, path: &TreePath) {
        unsafe {
            gtk_sys::gtk_icon_view_select_path(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn selected_foreach<P: FnMut(&IconView, &TreePath)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&IconView, &TreePath)>(icon_view: *mut gtk_sys::GtkIconView, path: *mut gtk_sys::GtkTreePath, data: glib_sys::gpointer) {
            let icon_view = from_glib_borrow(icon_view);
            let path = from_glib_borrow(path);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&icon_view, &path);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            gtk_sys::gtk_icon_view_selected_foreach(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            gtk_sys::gtk_icon_view_set_activate_on_single_click(self.as_ref().to_glib_none().0, single.to_glib());
        }
    }

    fn set_column_spacing(&self, column_spacing: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_column_spacing(self.as_ref().to_glib_none().0, column_spacing);
        }
    }

    fn set_columns(&self, columns: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_columns(self.as_ref().to_glib_none().0, columns);
        }
    }

    fn set_cursor<P: IsA<CellRenderer>>(&self, path: &TreePath, cell: Option<&P>, start_editing: bool) {
        unsafe {
            gtk_sys::gtk_icon_view_set_cursor(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0), cell.map(|p| p.as_ref()).to_glib_none().0, start_editing.to_glib());
        }
    }

    fn set_drag_dest_item(&self, path: Option<&TreePath>, pos: IconViewDropPosition) {
        unsafe {
            gtk_sys::gtk_icon_view_set_drag_dest_item(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0), pos.to_glib());
        }
    }

    fn set_item_orientation(&self, orientation: Orientation) {
        unsafe {
            gtk_sys::gtk_icon_view_set_item_orientation(self.as_ref().to_glib_none().0, orientation.to_glib());
        }
    }

    fn set_item_padding(&self, item_padding: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_item_padding(self.as_ref().to_glib_none().0, item_padding);
        }
    }

    fn set_item_width(&self, item_width: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_item_width(self.as_ref().to_glib_none().0, item_width);
        }
    }

    fn set_margin(&self, margin: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_margin(self.as_ref().to_glib_none().0, margin);
        }
    }

    fn set_markup_column(&self, column: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_markup_column(self.as_ref().to_glib_none().0, column);
        }
    }

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_icon_view_set_model(self.as_ref().to_glib_none().0, model.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_pixbuf_column(&self, column: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_pixbuf_column(self.as_ref().to_glib_none().0, column);
        }
    }

    fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            gtk_sys::gtk_icon_view_set_reorderable(self.as_ref().to_glib_none().0, reorderable.to_glib());
        }
    }

    fn set_row_spacing(&self, row_spacing: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_row_spacing(self.as_ref().to_glib_none().0, row_spacing);
        }
    }

    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            gtk_sys::gtk_icon_view_set_selection_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_text_column(&self, column: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_text_column(self.as_ref().to_glib_none().0, column);
        }
    }

    fn set_tooltip_cell<P: IsA<CellRenderer>>(&self, tooltip: &Tooltip, path: &TreePath, cell: Option<&P>) {
        unsafe {
            gtk_sys::gtk_icon_view_set_tooltip_cell(self.as_ref().to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0), cell.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_tooltip_column(&self, column: i32) {
        unsafe {
            gtk_sys::gtk_icon_view_set_tooltip_column(self.as_ref().to_glib_none().0, column);
        }
    }

    fn set_tooltip_item(&self, tooltip: &Tooltip, path: &TreePath) {
        unsafe {
            gtk_sys::gtk_icon_view_set_tooltip_item(self.as_ref().to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn unselect_all(&self) {
        unsafe {
            gtk_sys::gtk_icon_view_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_path(&self, path: &TreePath) {
        unsafe {
            gtk_sys::gtk_icon_view_unselect_path(self.as_ref().to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn unset_model_drag_dest(&self) {
        unsafe {
            gtk_sys::gtk_icon_view_unset_model_drag_dest(self.as_ref().to_glib_none().0);
        }
    }

    fn unset_model_drag_source(&self) {
        unsafe {
            gtk_sys::gtk_icon_view_unset_model_drag_source(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = Value::from_type(<CellArea as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"cell-area\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_activate_cursor_item<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-cursor-item\0".as_ptr() as *const _,
                Some(transmute(activate_cursor_item_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate_cursor_item(&self) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("activate-cursor-item", &[]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_item_activated<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"item-activated\0".as_ptr() as *const _,
                Some(transmute(item_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-cursor\0".as_ptr() as *const _,
                Some(transmute(move_cursor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_cursor(&self, step: MovementStep, count: i32) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-cursor", &[&step, &count]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"select-all\0".as_ptr() as *const _,
                Some(transmute(select_all_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_select_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("select-all", &[]).unwrap() };
    }

    fn connect_select_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"select-cursor-item\0".as_ptr() as *const _,
                Some(transmute(select_cursor_item_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_select_cursor_item(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("select-cursor-item", &[]).unwrap() };
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"selection-changed\0".as_ptr() as *const _,
                Some(transmute(selection_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_toggle_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"toggle-cursor-item\0".as_ptr() as *const _,
                Some(transmute(toggle_cursor_item_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_toggle_cursor_item(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("toggle-cursor-item", &[]).unwrap() };
    }

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"unselect-all\0".as_ptr() as *const _,
                Some(transmute(unselect_all_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_unselect_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("unselect-all", &[]).unwrap() };
    }

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::activate-on-single-click\0".as_ptr() as *const _,
                Some(transmute(notify_activate_on_single_click_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::column-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_column_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_columns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::columns\0".as_ptr() as *const _,
                Some(transmute(notify_columns_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_item_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::item-orientation\0".as_ptr() as *const _,
                Some(transmute(notify_item_orientation_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_item_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::item-padding\0".as_ptr() as *const _,
                Some(transmute(notify_item_padding_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_item_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::item-width\0".as_ptr() as *const _,
                Some(transmute(notify_item_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::margin\0".as_ptr() as *const _,
                Some(transmute(notify_margin_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_markup_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::markup-column\0".as_ptr() as *const _,
                Some(transmute(notify_markup_column_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::model\0".as_ptr() as *const _,
                Some(transmute(notify_model_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pixbuf_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pixbuf-column\0".as_ptr() as *const _,
                Some(transmute(notify_pixbuf_column_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reorderable\0".as_ptr() as *const _,
                Some(transmute(notify_reorderable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::row-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_row_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-mode\0".as_ptr() as *const _,
                Some(transmute(notify_selection_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute(notify_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-column\0".as_ptr() as *const _,
                Some(transmute(notify_text_column_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tooltip_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-column\0".as_ptr() as *const _,
                Some(transmute(notify_tooltip_column_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_cursor_item_trampoline<P, F: Fn(&P) -> bool + 'static>(this: *mut gtk_sys::GtkIconView, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast()).to_glib()
}

unsafe extern "C" fn item_activated_trampoline<P, F: Fn(&P, &TreePath) + 'static>(this: *mut gtk_sys::GtkIconView, path: *mut gtk_sys::GtkTreePath, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(path))
}

unsafe extern "C" fn move_cursor_trampoline<P, F: Fn(&P, MovementStep, i32) -> bool + 'static>(this: *mut gtk_sys::GtkIconView, step: gtk_sys::GtkMovementStep, count: libc::c_int, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast(), from_glib(step), count).to_glib()
}

unsafe extern "C" fn select_all_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn select_cursor_item_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn selection_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn toggle_cursor_item_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn unselect_all_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_activate_on_single_click_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_columns_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_item_orientation_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_item_padding_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_item_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_margin_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_markup_column_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pixbuf_column_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_reorderable_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_column_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tooltip_column_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIconView, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IconView> {
    let f: &F = &*(f as *const F);
    f(&IconView::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for IconView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconView")
    }
}
