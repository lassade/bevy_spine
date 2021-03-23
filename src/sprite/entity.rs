use bevy::{
    asset::Handle,
    ecs::Bundle,
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
use crate::transform::{GlobalMatrix, ShearTransform};

#[derive(Bundle)]
pub struct SpriteBundle {
    pub sprite: Handle<Sprite>,
    pub sprite_instance: SpriteInstance,
    pub mesh: Handle<Mesh>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub shear: ShearTransform,
    pub transform: Transform,
    pub global_matrix: GlobalMatrix,
}

impl Default for SpriteBundle {
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
            shear: Default::default(),
            transform: Default::default(),
            global_matrix: Default::default(),
        }
    }
}
