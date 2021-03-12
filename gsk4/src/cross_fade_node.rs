// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    CrossFadeNode,
    ffi::GskCrossFadeNode,
    ffi::gsk_cross_fade_node_get_type,
    RenderNodeType::CrossFadeNode
);

impl CrossFadeNode {
    #[doc(alias = "gsk_cross_fade_node_new")]
    pub fn new<P: IsRenderNode, Q: IsRenderNode>(
        start: &P,
        end: &Q,
        progress: f32,
    ) -> CrossFadeNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_cross_fade_node_new(
                start.as_ref().to_glib_none().0,
                end.as_ref().to_glib_none().0,
                progress,
            ))
        }
    }

    #[doc(alias = "gsk_cross_fade_node_get_end_child")]
    pub fn get_end_child(&self) -> Option<RenderNode> {
        unsafe {
            from_glib_none(ffi::gsk_cross_fade_node_get_end_child(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_cross_fade_node_get_progress")]
    pub fn get_progress(&self) -> f32 {
        unsafe { ffi::gsk_cross_fade_node_get_progress(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_cross_fade_node_get_start_child")]
    pub fn get_start_child(&self) -> Option<RenderNode> {
        unsafe {
            from_glib_none(ffi::gsk_cross_fade_node_get_start_child(
                self.to_glib_none().0,
            ))
        }
    }
}
