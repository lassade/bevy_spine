use bevy::prelude::*;

use crate::transform::{GlobalMatrix, ShearTransform};

#[derive(Bundle)]
pub struct BoneBundle {
    pub name: Name,
    pub parent: Parent,
    //pub color: Color,
    pub children: Children,
    pub shear: ShearTransform,
    pub transform: Transform,
    pub global_transform_matrix: GlobalMatrix,
}

impl Default for BoneBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            // TODO: Not a very good solution
            parent: Parent(Entity::new(u32::MAX)),
            //color: Default::default(),
            children: Default::default(),
            shear: Default::default(),
            transform: Default::default(),
            global_transform_matrix: Default::default(),
        }
    }
}
