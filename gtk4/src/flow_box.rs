// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FlowBox;
use glib::translate::*;
use std::ptr;

impl FlowBox {
    #[doc(alias = "gtk_flow_box_bind_model")]
    pub fn unbind_model(&self) {
        unsafe {
            ffi::gtk_flow_box_bind_model(
                self.to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
            )
        }
    }
}
