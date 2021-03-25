use std::ops::Mul;

use bevy::{
    prelude::*,
    render::renderer::{
        RenderResource, RenderResourceIterator, RenderResourceType, RenderResources,
    },
};

use super::Transform2D;

/// 2D analogue of [`LocalToWorld`](super::LocalToWorld)
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct LocalToWorld2D(pub Mat3);

impl LocalToWorld2D {
    #[inline]
    pub fn mul_transform(&self, transform: Transform2D) -> Self {
        LocalToWorld2D(self.0.mul_mat3(&transform.compute_matrix()))
    }
}

impl Default for LocalToWorld2D {
    #[inline]
    fn default() -> Self {
        Self(Mat3::IDENTITY)
    }
}

impl Mul for LocalToWorld2D {
    type Output = LocalToWorld2D;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        LocalToWorld2D(self.0.mul_mat3(&rhs.0))
    }
}

impl From<Mat3> for LocalToWorld2D {
    #[inline]
    fn from(value: Mat3) -> Self {
        Self(value)
    }
}

impl From<Transform2D> for LocalToWorld2D {
    #[inline]
    fn from(value: Transform2D) -> Self {
        LocalToWorld2D::from(value.compute_matrix())
    }
}

impl RenderResources for LocalToWorld2D {
    #[inline]
    fn render_resources_len(&self) -> usize {
        1
    }

    #[inline]
    fn get_render_resource(&self, _: usize) -> Option<&dyn RenderResource> {
        Some(self)
    }

    #[inline]
    fn get_render_resource_name(&self, _: usize) -> Option<&str> {
        Some("Transform2D")
    }

    #[inline]
    fn iter(&self) -> RenderResourceIterator {
        RenderResourceIterator::new(self)
    }
}

impl RenderResource for LocalToWorld2D {
    #[inline]
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    #[inline]
    fn buffer_byte_len(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }

    #[inline]
    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        // ! FIXME: [T; 9] doesn't impl `Byteable`
        let matrix: &[f32; 9] = self.0.as_ref();
        let len = std::mem::size_of_val(matrix);
        let bytes = unsafe { core::slice::from_raw_parts(matrix.as_ptr() as *const u8, len) };
        buffer[0..len].copy_from_slice(bytes);
    }

    #[inline]
    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}
