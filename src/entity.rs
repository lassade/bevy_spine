use bevy::prelude::*;

use crate::transform::{LocalToWorld, LocalToWorld2D, Transform2D};

pub type BoneBundle2D = BoneBundleBase<LocalToWorld2D>;

pub type BoneBundle2D5 = BoneBundleBase<LocalToWorld>;

#[derive(Bundle)]
pub struct BoneBundleBase<M: Send + Sync + 'static> {
    pub name: Name,
    pub parent: Parent,
    //pub color: Color,
    pub children: Children,
    pub transform: Transform2D,
    pub local_to_world: M,
}

impl<M> Default for BoneBundleBase<M>
where
    M: Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            name: Default::default(),
            // TODO: Not a very good solution
            parent: Parent(Entity::new(u32::MAX)),
            children: Default::default(),
            transform: Default::default(),
            local_to_world: Default::default(),
        }
    }
}
