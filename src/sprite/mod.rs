use bevy::{
    app::prelude::*,
    asset::{AddAsset, AssetStage, Assets, Handle},
    ecs::{IntoSystem, StageLabel, SystemStage},
    reflect::RegisterTypeBuilder,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        shader::{asset_shader_defs_system, Shader},
    },
};

mod entity;
mod mesh_helper;
mod render;
mod sprite;

pub use entity::*;
pub use render::*;
pub use sprite::{Sprite, *};

pub mod prelude {
    pub use super::{entity::SpriteBundle, Sprite, SpriteInstance};
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum SpriteStage {
    Update,
}

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Sprite>()
            .register_type::<SpriteInstance>()
            .add_stage_after(
                AssetStage::AssetEvents,
                SpriteStage::Update,
                SystemStage::parallel(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                asset_shader_defs_system::<Sprite>.system(),
            )
            .add_system_to_stage(
                SpriteStage::Update,
                sprite::rebuild_modified_sprite_system.system(),
            )
            .add_system_to_stage(SpriteStage::Update, sprite::update_sprite_system.system());

        let resources = app.resources_mut();

        // Add default sprite asset
        let mut sprites = resources.get_mut::<Assets<Sprite>>().unwrap();
        sprites.set_untracked(Handle::<Sprite>::default(), Sprite::default());

        // Set the pipeline and rendering
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        render_graph.add_system_node(
            node::SPRITE_INSTANCE,
            RenderResourcesNode::<SpriteInstance>::new(true),
        );
        render_graph
            .add_node_edge(node::SPRITE_INSTANCE, base::node::MAIN_PASS)
            .unwrap();

        render_graph.add_system_node(node::SPRITE, AssetRenderResourcesNode::<Sprite>::new(false));
        render_graph
            .add_node_edge(node::SPRITE, base::node::MAIN_PASS)
            .unwrap();

        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
        pipelines.set_untracked(SPRITE_PIPELINE_HANDLE, build_sprite_pipeline(&mut shaders));
    }
}
