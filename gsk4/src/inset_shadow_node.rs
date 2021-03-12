// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RoundedRect};
use glib::translate::*;

define_render_node!(
    InsetShadowNode,
    ffi::GskInsetShadowNode,
    ffi::gsk_inset_shadow_node_get_type,
    RenderNodeType::InsetShadowNode
);

impl InsetShadowNode {
    #[doc(alias = "gsk_inset_shadow_node_new")]
    pub fn new(
        outline: &RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) -> InsetShadowNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_inset_shadow_node_new(
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            ))
        }
    }

    #[doc(alias = "gsk_inset_shadow_node_get_blur_radius")]
    pub fn get_blur_radius(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_blur_radius(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_inset_shadow_node_get_color")]
    pub fn get_color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_inset_shadow_node_get_color(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_inset_shadow_node_get_dx")]
    pub fn get_dx(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_dx(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_inset_shadow_node_get_dy")]
    pub fn get_dy(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_dy(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_inset_shadow_node_get_outline")]
    pub fn get_outline(&self) -> Option<RoundedRect> {
        unsafe {
            from_glib_none(ffi::gsk_inset_shadow_node_get_outline(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_inset_shadow_node_get_spread")]
    pub fn get_spread(&self) -> f32 {
        unsafe { ffi::gsk_inset_shadow_node_get_spread(self.to_glib_none().0) }
    }
}
