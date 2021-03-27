use bevy::prelude::*;

/// Doesn't propagate transform for any child of this component;
///
/// A constraint often need the parent [`WorldToLocal`] or the their targets [`LocalToWorld`] matrices
/// to update the entity transform that have to be propagate down the hierarchy chain. So what [`DontPropagateTransform`]
/// do is to stop the transform system to do redundant work by propagating transformations that will be
/// discarded anyway.
#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct DontPropagateTransform;
