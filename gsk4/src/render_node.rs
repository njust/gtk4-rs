// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNodeType;
use glib::translate::*;
use glib::{StaticType, Type};
use std::fmt;
use std::path::Path;
use std::ptr;

// Can't use get_type here as this is not a boxed type but another fundamental type
glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RenderNode(Shared<ffi::GskRenderNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr),
        unref => |ptr| ffi::gsk_render_node_unref(ptr),
    }
}

impl StaticType for RenderNode {
    #[doc(alias = "gsk_render_node_type_get_type")]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gsk_render_node_type_get_type()) }
    }
}

impl RenderNode {
    //#[doc(alias = "gsk_render_node_deserialize")]
    //pub fn deserialize(bytes: &glib::Bytes, error_func: /*Unimplemented*/FnMut(/*Ignored*/ParseLocation, /*Ignored*/ParseLocation, &glib::Error), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<RenderNode> {
    //    unsafe { TODO: call ffi:gsk_render_node_deserialize() }
    //}

    pub fn downcast<T: IsRenderNode>(self) -> Result<T, RenderNode> {
        unsafe {
            if self.get_node_type() == T::NODE_TYPE {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: IsRenderNode>(&self) -> Option<&T> {
        unsafe {
            if self.get_node_type() == T::NODE_TYPE {
                Some(&*(self as *const RenderNode as *const T))
            } else {
                None
            }
        }
    }

    pub fn draw(&self, cr: &cairo::Context) {
        unsafe {
            ffi::gsk_render_node_draw(self.to_glib_none().0, mut_override(cr.to_glib_none().0));
        }
    }

    pub fn get_bounds(&self) -> graphene::Rect {
        unsafe {
            let mut bounds = graphene::Rect::uninitialized();
            ffi::gsk_render_node_get_bounds(self.to_glib_none().0, bounds.to_glib_none_mut().0);
            bounds
        }
    }

    pub fn get_node_type(&self) -> RenderNodeType {
        unsafe { from_glib(ffi::gsk_render_node_get_node_type(self.to_glib_none().0)) }
    }

    pub fn serialize(&self) -> glib::Bytes {
        unsafe { from_glib_full(ffi::gsk_render_node_serialize(self.to_glib_none().0)) }
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gsk_render_node_write_to_file(
                self.to_glib_none().0,
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait IsRenderNode:
    StaticType
    + FromGlibPtrFull<*mut ffi::GskRenderNode>
    + std::convert::AsRef<crate::RenderNode>
    + 'static
{
    const NODE_TYPE: RenderNodeType;
    fn upcast(self) -> RenderNode;
    fn upcast_ref(&self) -> &RenderNode;
}

pub const NONE_RENDER_NODE: Option<&RenderNode> = None;

impl fmt::Display for RenderNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RenderNode")
    }
}

macro_rules! define_render_node {
    ($rust_type:ident, $ffi_type:path, $get_type:path, $node_type:path) => {
        // Can't use get_type here as this is not a boxed type but another fundamental type
        glib::wrapper! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $rust_type(Shared<$ffi_type>);

            match fn {
                ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode) as *mut $ffi_type,
                unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
            }
        }

        impl ::glib::StaticType for $rust_type {
            fn static_type() -> ::glib::Type {
                unsafe { from_glib($get_type()) }
            }
        }

        impl std::convert::AsRef<crate::RenderNode> for $rust_type {
            fn as_ref(&self) -> &crate::RenderNode {
                &*self
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = crate::RenderNode;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    &*(self as *const $rust_type as *const crate::RenderNode)
                }
            }
        }

        impl crate::render_node::IsRenderNode for $rust_type {
            const NODE_TYPE: RenderNodeType = $node_type;

            fn upcast(self) -> crate::RenderNode {
                unsafe { from_glib_full(self.to_glib_full() as *mut ffi::GskRenderNode) }
            }

            fn upcast_ref(&self) -> &crate::RenderNode {
                &*self
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut ffi::GskRenderNode> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut ffi::GskRenderNode) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl ::std::fmt::Display for $rust_type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(::std::stringify!($rust_type))
            }
        }
    };
}
