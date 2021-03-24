use std::ops::Mul;

use bevy::{
    core::Bytes,
    prelude::*,
    render::renderer::{
        RenderResource, RenderResourceIterator, RenderResourceType, RenderResources,
    },
};

use super::LocalToWorld2D;

/// Entity model matrix, similar to the [`GlobalTransform`] but uses a [`Mat4`] under the hood;
///
/// **WARNING** [`LocalToWorld`] is incompatible with [`GlobalTransform`],
/// both will set the `"Transform"` uniform;
///
/// **WARNING** [`LocalToWorld`] can be a children of [`GlobalTransform`]
/// but not the other way around;
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct LocalToWorld(pub Mat4);

impl LocalToWorld {
    #[inline]
    pub fn mul_transform(&self, transform: Transform) -> Self {
        LocalToWorld(self.0.mul_mat4(&transform.compute_matrix()))
    }
}

impl Default for LocalToWorld {
    #[inline]
    fn default() -> Self {
        Self(Mat4::identity())
    }
}

impl Mul for LocalToWorld {
    type Output = LocalToWorld;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        LocalToWorld(self.0.mul_mat4(&rhs.0))
    }
}

impl From<LocalToWorld2D> for LocalToWorld {
    #[inline]
    fn from(value: LocalToWorld2D) -> Self {
        LocalToWorld(Mat4::from_cols(
            Vec2::from(value.0.x_axis).extend(0.0).extend(0.0),
            Vec2::from(value.0.y_axis).extend(0.0).extend(0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec2::from(value.0.z_axis).extend(0.0).extend(1.0),
        ))
    }
}

impl From<Mat4> for LocalToWorld {
    #[inline]
    fn from(value: Mat4) -> Self {
        Self(value)
    }
}

impl From<Transform> for LocalToWorld {
    #[inline]
    fn from(value: Transform) -> Self {
        LocalToWorld::from(value.compute_matrix())
    }
}

impl RenderResources for LocalToWorld {
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
        Some("Transform")
    }

    #[inline]
    fn iter(&self) -> RenderResourceIterator {
        RenderResourceIterator::new(self)
    }
}

impl RenderResource for LocalToWorld {
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
        self.0.write_bytes(buffer);
    }

    #[inline]
    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}
