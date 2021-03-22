use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};

macro_rules! mesh_attr {
    ($mesh:tt , $attr:expr, $var:path) => {
        if let Some($var(buffer)) = $mesh.attribute_mut($attr) {
            &mut *(buffer as *mut _)
        } else {
            $mesh.set_attribute($attr, $var(vec![]));
            if let Some($var(buffer)) = $mesh.attribute_mut($attr) {
                &mut *(buffer as *mut _)
            } else {
                unreachable!()
            }
        }
    };
}

macro_rules! mesh_index {
    ($mesh:tt, $var:path) => {
        if let Some($var(buffer)) = $mesh.indices_mut() {
            &mut *(buffer as *mut _)
        } else {
            $mesh.set_indices(Some($var(vec![])));
            if let Some($var(buffer)) = $mesh.indices_mut() {
                &mut *(buffer as *mut _)
            } else {
                unreachable!()
            }
        }
    };
}

pub struct MeshEditXU<'a> {
    pub vertices: &'a mut Vec<[f32; 3]>,
    pub normals: &'a mut Vec<[f32; 3]>,
    pub uvs: &'a mut Vec<[f32; 2]>,
    pub indices: &'a mut Vec<u32>,
    _mesh: &'a mut Mesh,
}

impl<'a> From<&'a mut Mesh> for MeshEditXU<'a> {
    fn from(mesh: &'a mut Mesh) -> Self {
        unsafe {
            Self {
                vertices: mesh_attr!(
                    mesh,
                    Mesh::ATTRIBUTE_POSITION,
                    VertexAttributeValues::Float3
                ),
                normals: mesh_attr!(mesh, Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float3),
                uvs: mesh_attr!(mesh, Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2),
                indices: mesh_index!(mesh, Indices::U32),
                _mesh: mesh,
            }
        }
    }
}
