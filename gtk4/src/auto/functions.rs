// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleProperty;
use crate::AccessibleRelation;
use crate::AccessibleRole;
use crate::AccessibleState;
use crate::DebugFlags;
use crate::PageSetup;
use crate::PrintSettings;
use crate::StyleContext;
use crate::TextDirection;
use crate::TreeModel;
use crate::TreePath;
use crate::Widget;
use crate::Window;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::ptr;

#[doc(alias = "gtk_accelerator_get_default_mod_mask")]
pub fn accelerator_get_default_mod_mask() -> gdk::ModifierType {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_accelerator_get_default_mod_mask()) }
}

#[doc(alias = "gtk_accelerator_get_label")]
pub fn accelerator_get_label(
    accelerator_key: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label(
            accelerator_key,
            accelerator_mods.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_get_label_with_keycode")]
pub fn accelerator_get_label_with_keycode(
    display: Option<&gdk::Display>,
    accelerator_key: u32,
    keycode: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label_with_keycode(
            display.to_glib_none().0,
            accelerator_key,
            keycode,
            accelerator_mods.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_name")]
pub fn accelerator_name(
    accelerator_key: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name(
            accelerator_key,
            accelerator_mods.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_name_with_keycode")]
pub fn accelerator_name_with_keycode(
    display: Option<&gdk::Display>,
    accelerator_key: u32,
    keycode: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name_with_keycode(
            display.to_glib_none().0,
            accelerator_key,
            keycode,
            accelerator_mods.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_parse")]
pub fn accelerator_parse(accelerator: &str) -> Option<(u32, gdk::ModifierType)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut accelerator_key = mem::MaybeUninit::uninit();
        let mut accelerator_mods = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gtk_accelerator_parse(
            accelerator.to_glib_none().0,
            accelerator_key.as_mut_ptr(),
            accelerator_mods.as_mut_ptr(),
        ));
        let accelerator_key = accelerator_key.assume_init();
        let accelerator_mods = accelerator_mods.assume_init();
        if ret {
            Some((accelerator_key, from_glib(accelerator_mods)))
        } else {
            None
        }
    }
}

//#[doc(alias = "gtk_accelerator_parse_with_keycode")]
//pub fn accelerator_parse_with_keycode(accelerator: &str, display: Option<&gdk::Display>, accelerator_codes: Vec<u32>) -> Option<(u32, gdk::ModifierType)> {
//    unsafe { TODO: call ffi:gtk_accelerator_parse_with_keycode() }
//}

#[doc(alias = "gtk_check_version")]
pub fn check_version(
    required_major: u32,
    required_minor: u32,
    required_micro: u32,
) -> Option<glib::GString> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_check_version(
            required_major,
            required_minor,
            required_micro,
        ))
    }
}

#[doc(alias = "gtk_css_parser_warning_quark")]
pub fn css_parser_warning_quark() -> glib::Quark {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_css_parser_warning_quark()) }
}

#[doc(alias = "gtk_disable_setlocale")]
pub fn disable_setlocale() {
    assert_not_initialized!();
    unsafe {
        ffi::gtk_disable_setlocale();
    }
}

//#[doc(alias = "gtk_distribute_natural_allocation")]
//pub fn distribute_natural_allocation(extra_space: i32, n_requested_sizes: u32, sizes: /*Ignored*/&mut RequestedSize) -> i32 {
//    unsafe { TODO: call ffi:gtk_distribute_natural_allocation() }
//}

#[doc(alias = "gtk_get_binary_age")]
pub fn get_binary_age() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_binary_age() }
}

#[doc(alias = "gtk_get_debug_flags")]
pub fn get_debug_flags() -> DebugFlags {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_get_debug_flags()) }
}

#[doc(alias = "gtk_get_default_language")]
pub fn get_default_language() -> Option<pango::Language> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gtk_get_default_language()) }
}

#[doc(alias = "gtk_get_interface_age")]
pub fn get_interface_age() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_interface_age() }
}

#[doc(alias = "gtk_get_locale_direction")]
pub fn get_locale_direction() -> TextDirection {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_get_locale_direction()) }
}

#[doc(alias = "gtk_get_major_version")]
pub fn get_major_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_major_version() }
}

#[doc(alias = "gtk_get_micro_version")]
pub fn get_micro_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_micro_version() }
}

#[doc(alias = "gtk_get_minor_version")]
pub fn get_minor_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_minor_version() }
}

#[doc(alias = "gtk_hsv_to_rgb")]
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut r = mem::MaybeUninit::uninit();
        let mut g = mem::MaybeUninit::uninit();
        let mut b = mem::MaybeUninit::uninit();
        ffi::gtk_hsv_to_rgb(h, s, v, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr());
        let r = r.assume_init();
        let g = g.assume_init();
        let b = b.assume_init();
        (r, g, b)
    }
}

#[doc(alias = "gtk_print_run_page_setup_dialog")]
pub fn print_run_page_setup_dialog<P: IsA<Window>>(
    parent: Option<&P>,
    page_setup: Option<&PageSetup>,
    settings: &PrintSettings,
) -> Option<PageSetup> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_print_run_page_setup_dialog(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            page_setup.to_glib_none().0,
            settings.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gtk_print_run_page_setup_dialog_async")]
pub fn print_run_page_setup_dialog_async<
    P: IsA<Window>,
    Q: FnOnce(&PageSetup) + Send + Sync + 'static,
>(
    parent: Option<&P>,
    page_setup: Option<&PageSetup>,
    settings: &PrintSettings,
    done_cb: Q,
) {
    skip_assert_initialized!();
    let done_cb_data: Box_<Q> = Box_::new(done_cb);
    unsafe extern "C" fn done_cb_func<
        P: IsA<Window>,
        Q: FnOnce(&PageSetup) + Send + Sync + 'static,
    >(
        page_setup: *mut ffi::GtkPageSetup,
        data: glib::ffi::gpointer,
    ) {
        let page_setup = from_glib_borrow(page_setup);
        let callback: Box_<Q> = Box_::from_raw(data as *mut _);
        (*callback)(&page_setup);
    }
    let done_cb = Some(done_cb_func::<P, Q> as _);
    let super_callback0: Box_<Q> = done_cb_data;
    unsafe {
        ffi::gtk_print_run_page_setup_dialog_async(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            page_setup.to_glib_none().0,
            settings.to_glib_none().0,
            done_cb,
            Box_::into_raw(super_callback0) as *mut _,
        );
    }
}

#[doc(alias = "gtk_render_activity")]
pub fn render_activity<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_activity(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_arrow")]
pub fn render_arrow<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    angle: f64,
    x: f64,
    y: f64,
    size: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_arrow(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            angle,
            x,
            y,
            size,
        );
    }
}

#[doc(alias = "gtk_render_background")]
pub fn render_background<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_background(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_check")]
pub fn render_check<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_check(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_expander")]
pub fn render_expander<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_expander(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_focus")]
pub fn render_focus<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_focus(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_frame")]
pub fn render_frame<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_frame(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_handle")]
pub fn render_handle<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_handle(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_render_icon")]
pub fn render_icon<P: IsA<StyleContext>, Q: IsA<gdk::Texture>>(
    context: &P,
    cr: &cairo::Context,
    texture: &Q,
    x: f64,
    y: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_icon(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            texture.as_ref().to_glib_none().0,
            x,
            y,
        );
    }
}

#[doc(alias = "gtk_render_layout")]
pub fn render_layout<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    layout: &pango::Layout,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_layout(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            layout.to_glib_none().0,
        );
    }
}

#[doc(alias = "gtk_render_line")]
pub fn render_line<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_line(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x0,
            y0,
            x1,
            y1,
        );
    }
}

#[doc(alias = "gtk_render_option")]
pub fn render_option<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_option(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_rgb_to_hsv")]
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut h = mem::MaybeUninit::uninit();
        let mut s = mem::MaybeUninit::uninit();
        let mut v = mem::MaybeUninit::uninit();
        ffi::gtk_rgb_to_hsv(r, g, b, h.as_mut_ptr(), s.as_mut_ptr(), v.as_mut_ptr());
        let h = h.assume_init();
        let s = s.assume_init();
        let v = v.assume_init();
        (h, s, v)
    }
}

#[doc(alias = "gtk_set_debug_flags")]
pub fn set_debug_flags(flags: DebugFlags) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_set_debug_flags(flags.to_glib());
    }
}

#[doc(alias = "gtk_show_uri")]
pub fn show_uri<P: IsA<Window>>(parent: Option<&P>, uri: &str, timestamp: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_show_uri(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            uri.to_glib_none().0,
            timestamp,
        );
    }
}

#[doc(alias = "gtk_test_accessible_assertion_message_role")]
pub fn test_accessible_assertion_message_role<P: IsA<Accessible>>(
    domain: &str,
    file: &str,
    line: i32,
    func: &str,
    expr: &str,
    accessible: &P,
    expected_role: AccessibleRole,
    actual_role: AccessibleRole,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_accessible_assertion_message_role(
            domain.to_glib_none().0,
            file.to_glib_none().0,
            line,
            func.to_glib_none().0,
            expr.to_glib_none().0,
            accessible.as_ref().to_glib_none().0,
            expected_role.to_glib(),
            actual_role.to_glib(),
        );
    }
}

//#[doc(alias = "gtk_test_accessible_check_property")]
//pub fn test_accessible_check_property<P: IsA<Accessible>>(accessible: &P, property: AccessibleProperty, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:gtk_test_accessible_check_property() }
//}

//#[doc(alias = "gtk_test_accessible_check_relation")]
//pub fn test_accessible_check_relation<P: IsA<Accessible>>(accessible: &P, relation: AccessibleRelation, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:gtk_test_accessible_check_relation() }
//}

//#[doc(alias = "gtk_test_accessible_check_state")]
//pub fn test_accessible_check_state<P: IsA<Accessible>>(accessible: &P, state: AccessibleState, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:gtk_test_accessible_check_state() }
//}

#[doc(alias = "gtk_test_accessible_has_property")]
pub fn test_accessible_has_property<P: IsA<Accessible>>(
    accessible: &P,
    property: AccessibleProperty,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_property(
            accessible.as_ref().to_glib_none().0,
            property.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_test_accessible_has_relation")]
pub fn test_accessible_has_relation<P: IsA<Accessible>>(
    accessible: &P,
    relation: AccessibleRelation,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_relation(
            accessible.as_ref().to_glib_none().0,
            relation.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_test_accessible_has_role")]
pub fn test_accessible_has_role<P: IsA<Accessible>>(accessible: &P, role: AccessibleRole) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_role(
            accessible.as_ref().to_glib_none().0,
            role.to_glib(),
        ))
    }
}

#[doc(alias = "gtk_test_accessible_has_state")]
pub fn test_accessible_has_state<P: IsA<Accessible>>(
    accessible: &P,
    state: AccessibleState,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_state(
            accessible.as_ref().to_glib_none().0,
            state.to_glib(),
        ))
    }
}

//#[doc(alias = "gtk_test_init")]
//pub fn test_init(argvp: /*Unimplemented*/Vec<glib::GString>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:gtk_test_init() }
//}

//#[doc(alias = "gtk_test_list_all_types")]
//pub fn test_list_all_types() -> /*Unimplemented*/CArray TypeId { ns_id: 0, id: 30 } {
//    unsafe { TODO: call ffi:gtk_test_list_all_types() }
//}

#[doc(alias = "gtk_test_register_all_types")]
pub fn test_register_all_types() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_test_register_all_types();
    }
}

#[doc(alias = "gtk_test_widget_wait_for_draw")]
pub fn test_widget_wait_for_draw<P: IsA<Widget>>(widget: &P) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_widget_wait_for_draw(widget.as_ref().to_glib_none().0);
    }
}

#[doc(alias = "gtk_tree_create_row_drag_content")]
pub fn tree_create_row_drag_content<P: IsA<TreeModel>>(
    tree_model: &P,
    path: &mut TreePath,
) -> Option<gdk::ContentProvider> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_tree_create_row_drag_content(
            tree_model.as_ref().to_glib_none().0,
            path.to_glib_none_mut().0,
        ))
    }
}

#[doc(alias = "gtk_tree_get_row_drag_data")]
pub fn tree_get_row_drag_data(
    value: &glib::Value,
) -> Option<(Option<TreeModel>, Option<TreePath>)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut tree_model = ptr::null_mut();
        let mut path = ptr::null_mut();
        let ret = from_glib(ffi::gtk_tree_get_row_drag_data(
            value.to_glib_none().0,
            &mut tree_model,
            &mut path,
        ));
        if ret {
            Some((from_glib_none(tree_model), from_glib_full(path)))
        } else {
            None
        }
    }
}
