use bevy::prelude::{Bundle, Transform};

use super::{LocalToWorld, LocalToWorld2D, Transform2D};

pub type TransformBundle2D = TransformBundleBase<Transform2D, LocalToWorld2D>;

pub type TransformBundle2D5 = TransformBundleBase<Transform2D, LocalToWorld>;

pub type TransformBundle = TransformBundleBase<Transform, LocalToWorld>;

#[derive(Bundle)]
pub struct TransformBundleBase<T: Send + Sync + 'static, M: Send + Sync + 'static> {
    pub transform: T,
    pub local_to_world: M,
}

impl<T, M> Default for TransformBundleBase<T, M>
where
    T: Default + Send + Sync + 'static,
    M: Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            transform: Default::default(),
            local_to_world: Default::default(),
        }
    }
}
