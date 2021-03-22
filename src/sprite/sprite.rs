use bevy::{
    asset::Asset,
    core::Bytes,
    prelude::*,
    reflect::{Reflect, ReflectComponent, TypeUuid},
    render::{
        mesh::Indices,
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::ShaderDefs,
    },
    utils::HashSet,
};

use super::mesh_helper::MeshEditXU;

#[derive(Debug)]
pub enum Rotation {
    None,
    CW,
    CCW,
}

#[derive(Debug)]
pub enum By<T: Asset> {
    Value(T),
    Handle(Handle<T>),
}

#[derive(Debug)]
pub enum SpriteShape {
    Rect {
        /// Texture rectangle lower-left corner
        min: Vec2,
        /// Texture rectangle top-right corner
        max: Vec2,
        /// Texture rectangle orientation
        rotation: Rotation,
        /// Size in world units
        size: Vec2,
        /// Normalized sprite pivot
        pivot: Vec2,
        /// 9 Slice padding
        padding: Option<Vec4>,
    },
    Custom {
        /// Mesh will be copied over to the underling sprite mesh
        mesh: By<Mesh>,
    },
}

impl Default for SpriteShape {
    fn default() -> Self {
        SpriteShape::Rect {
            min: Vec2::zero(),
            max: Vec2::splat(100.0),
            rotation: Rotation::None,
            size: Vec2::one(),
            pivot: Vec2::splat(0.5),
            padding: None,
        }
    }
}

#[derive(Default, Debug, RenderResources, TypeUuid, ShaderDefs)]
#[uuid = "8d3d1fed-e9e0-4695-96bd-75d2143cc376"]
pub struct Sprite {
    #[shader_def]
    pub texture: Option<Handle<Texture>>,
    pub color_base: Color,
    #[render_resources(ignore)]
    shape: SpriteShape,
    // ? NOTE: Create once and don't change, entities won't be able to update
    #[render_resources(ignore)]
    mesh: Option<Handle<Mesh>>,
}

impl Sprite {
    pub fn with_shape(texture: Option<Handle<Texture>>, shape: SpriteShape) -> Self {
        Self {
            texture,
            color_base: Default::default(),
            shape,
            mesh: None,
        }
    }

    pub const fn shape(&self) -> &SpriteShape {
        &self.shape
    }

    pub const fn mesh(&self) -> Option<&Handle<Mesh>> {
        self.mesh.as_ref()
    }
}

fn rebuild_mesh(sprite: &mut Sprite, meshes: &mut Assets<Mesh>) {
    let meshes_ptr = meshes as *mut _ as *const Assets<Mesh>;

    let mesh_target_handle = &*sprite
        .mesh
        .get_or_insert_with(|| meshes.add(Mesh::default()));
    let mesh_target = meshes.get_mut(mesh_target_handle).unwrap();

    match &mut sprite.shape {
        SpriteShape::Rect {
            min,
            max,
            rotation,
            size,
            pivot,
            padding,
        } => {
            let mesh_editable: MeshEditXU = mesh_target.into();

            if let Some(padding) = padding {
                let _ = padding;
                // TODO: 9-slice
                todo!("9-slice")
            } else {
                mesh_editable.uvs.resize(4, [0.0; 2]);
                mesh_editable.uvs[0] = [min.x, min.y];
                mesh_editable.uvs[1] = [max.x, min.y];
                mesh_editable.uvs[2] = [max.x, max.y];
                mesh_editable.uvs[3] = [min.x, max.y];

                mesh_editable.vertices.resize(4, [0.0; 3]);
                let center = *size * *pivot;
                let min = -center;
                let max = *size - center;
                mesh_editable.vertices[0] = [min.x, min.y, 0.0];
                mesh_editable.vertices[1] = [max.x, min.y, 0.0];
                mesh_editable.vertices[2] = [max.x, max.y, 0.0];
                mesh_editable.vertices[3] = [min.x, max.y, 0.0];

                mesh_editable.normals.resize(4, [0.0, 0.0, 1.0]);

                mesh_editable.indices.clear();
                mesh_editable
                    .indices
                    .extend_from_slice(&[0, 1, 2, 0, 2, 3][..]);

                match *rotation {
                    Rotation::None => {}
                    Rotation::CW => {
                        let temp = mesh_editable.uvs[3];
                        for i in (1..4).rev() {
                            // 3 <- 2
                            // 2 <- 1
                            // 1 <- 0
                            mesh_editable.uvs[i] = mesh_editable.uvs[i - 1];
                        }
                        // 0 <- 3*
                        mesh_editable.uvs[0] = temp;
                    }
                    Rotation::CCW => {
                        let temp = mesh_editable.uvs[0];
                        for i in 0..3 {
                            // 0 <- 1
                            // 1 <- 2
                            // 2 <- 3
                            mesh_editable.uvs[i] = mesh_editable.uvs[i + 1];
                        }
                        // 3 <- 0*
                        mesh_editable.uvs[3] = temp;
                    }
                }
            }
        }
        SpriteShape::Custom { mesh } => {
            match mesh {
                By::Value(mesh) => {
                    std::mem::swap(mesh_target, mesh);
                }
                By::Handle(ref mesh_source_handle) => {
                    if mesh_source_handle != mesh_target_handle {
                        // SAFETY: Source and target are different assets
                        let mesh_source =
                            unsafe { (&*meshes_ptr).get(mesh_source_handle).unwrap() };

                        if let Some(attr) = mesh_source.attribute(Mesh::ATTRIBUTE_POSITION) {
                            mesh_target.set_attribute(Mesh::ATTRIBUTE_POSITION, attr.clone());
                        }

                        match mesh_source.indices() {
                            Some(Indices::U16(indices)) => {
                                mesh_target.set_indices(Some(Indices::U16(indices.clone())))
                            }
                            Some(Indices::U32(indices)) => {
                                mesh_target.set_indices(Some(Indices::U32(indices.clone())))
                            }
                            None => mesh_target.set_indices(None),
                        }
                    }
                }
            }

            // Set the mesh handle
            *mesh = By::Handle(mesh_target_handle.clone());
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Reflect, RenderResources)]
#[render_resources(from_self)]
#[reflect(Component)]
pub struct SpriteInstance {
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl RenderResource for SpriteInstance {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(20)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        // Write the size buffer
        let (color_buffer, flip_buffer) = buffer.split_at_mut(16);
        self.color.write_bytes(color_buffer);

        // First bit means flip x, second bit means flip y
        flip_buffer[0] = if self.flip_x { 0b01 } else { 0 } | if self.flip_y { 0b10 } else { 0 };
        flip_buffer[1] = 0;
        flip_buffer[2] = 0;
        flip_buffer[3] = 0;
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}

///////////////////////////////////////////////////////////////////////////////

// After AssetEvents and before  update_mesh_system
pub(crate) fn rebuild_modified_sprite_system(
    mut meshes: ResMut<Assets<Mesh>>,
    sprites: ResMut<Assets<Sprite>>,
    mut sprite_events: EventReader<AssetEvent<Sprite>>,
) {
    let mut changed = HashSet::default();
    for event in sprite_events.iter() {
        match event {
            AssetEvent::Created { ref handle } => {
                changed.insert(handle.clone_weak());
            }
            AssetEvent::Modified { ref handle } => {
                changed.insert(handle.clone_weak());
            }
            AssetEvent::Removed { ref handle } => {
                changed.remove(handle);
            }
        }
    }

    let meshes = &mut *meshes;
    let sprites = &*sprites;
    for sprite_handle in changed.iter() {
        if let Some(sprite) = sprites.get(sprite_handle) {
            // SAFETY: System have mutability over `Assets<Sprite>`,
            // this is used to avoid triggering asset events that
            // will cause the sprite to be updated every frame
            let sprite = unsafe { &mut *(sprite as *const _ as *mut _) };
            rebuild_mesh(sprite, meshes);
            dbg!("changed");
        }
    }
}

pub(crate) fn update_sprite_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut query: Query<(&mut Handle<Mesh>, &Handle<Sprite>), Changed<Handle<Sprite>>>,
) {
    let meshes = &mut *meshes;
    for (mut mesh_handle, sprite_handle) in query.iter_mut() {
        if let Some(sprite) = sprites.get(sprite_handle) {
            *mesh_handle = if let Some(mesh) = &sprite.mesh {
                dbg!("set");
                mesh.clone()
            } else {
                let sprite: &mut Sprite = sprites.get_mut(sprite_handle).unwrap();
                rebuild_mesh(sprite, meshes);
                dbg!("missing");
                sprite.mesh.as_ref().unwrap().clone()
            };
        }
    }
}
