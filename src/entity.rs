use bevy::prelude::*;

#[derive(Bundle)]
pub struct BoneBundle {
    pub name: Name,
    pub parent: Parent,
    //pub color: Color,
    pub children: Children,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for BoneBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            // TODO: Not a very good solution
            parent: Parent(Entity::new(u32::MAX)),
            //color: Default::default(),
            children: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
