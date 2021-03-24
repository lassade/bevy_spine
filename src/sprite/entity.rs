use bevy::{
    asset::Handle,
    ecs::prelude::*,
    render::{
        mesh::Mesh,
        pipeline::{RenderPipeline, RenderPipelines},
        prelude::{Draw, Visible},
        render_graph::base::MainPass,
    },
    transform::prelude::Transform,
};

use super::{
    render::SPRITE_PIPELINE_HANDLE,
    sprite::{Sprite, SpriteInstance},
};
use crate::transform::{LocalToWorld, LocalToWorld2D, Transform2D};

pub type SpriteBundle2D = SpriteBundleBase<Transform2D, LocalToWorld2D>;

pub type SpriteBundle2D5 = SpriteBundleBase<Transform2D, LocalToWorld>;

pub type SpriteBundle = SpriteBundleBase<Transform, LocalToWorld>;

#[derive(Bundle)]
pub struct SpriteBundleBase<T: Send + Sync + 'static, M: Send + Sync + 'static> {
    pub sprite: Handle<Sprite>,
    pub sprite_instance: SpriteInstance,
    pub mesh: Handle<Mesh>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: T,
    pub local_to_world: M,
}

impl<T, M> Default for SpriteBundleBase<T, M>
where
    T: Default + Send + Sync + 'static,
    M: Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            sprite_instance: Default::default(),
            mesh: Default::default(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SPRITE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            draw: Default::default(),
            transform: Default::default(),
            local_to_world: Default::default(),
        }
    }
}
