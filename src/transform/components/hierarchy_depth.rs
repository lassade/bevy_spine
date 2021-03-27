use bevy::prelude::*;

#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct HierarchyDepth(usize);
