use bevy::{
    core::Bytes,
    prelude::*,
    render::renderer::{
        RenderResource, RenderResourceIterator, RenderResourceType, RenderResources,
    },
};

/// Entity model matrix, similar to the [`GlobalTransform`] but uses a [`Mat4`] under the hood;
///
/// **WARNING** [`GlobalMatrix`] is incompatible with [`GlobalTransform`],
/// both will set the `"Transform"` uniform;
///
/// **WARNING** [`GlobalMatrix`] can be a children of [`GlobalTransform`]
/// but not the other way around;
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct GlobalMatrix {
    pub value: Mat4,
}

impl Default for GlobalMatrix {
    #[inline]
    fn default() -> Self {
        Self {
            value: Mat4::identity(),
        }
    }
}

impl From<Mat4> for GlobalMatrix {
    #[inline]
    fn from(value: Mat4) -> Self {
        Self { value }
    }
}

impl From<GlobalTransform> for GlobalMatrix {
    #[inline]
    fn from(value: GlobalTransform) -> Self {
        GlobalMatrix::from(value.compute_matrix())
    }
}

impl RenderResources for GlobalMatrix {
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

impl RenderResource for GlobalMatrix {
    #[inline]
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    #[inline]
    fn buffer_byte_len(&self) -> Option<usize> {
        Some(4 * 16)
    }

    #[inline]
    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        self.value.write_bytes(buffer);
    }

    #[inline]
    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}
