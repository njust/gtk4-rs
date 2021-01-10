// Take a look at the license at the top of the repository in the LICENSE file.
use crate::RenderNode;
use glib::translate::*;

impl RenderNode {
    #[doc(alias = "gsk_render_node_deserialize")]
    pub fn deserialize(bytes: &glib::Bytes) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_render_node_deserialize(
                bytes.to_glib_none().0,
                None,
                std::ptr::null_mut(),
            ))
        }
    }
}
