use bevy::{
    asset::AssetStage,
    prelude::*,
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

// ? NOTE: SpriteBundle have the same name as the bevy_sprite
pub use entity::{SpriteBundle, *};
pub use render::*;
pub use sprite::{Sprite, *};

pub mod prelude {
    pub use super::{entity::SpriteBundleBase, Sprite, SpriteInstance};
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum SpriteStage {
    Update,
}

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Sprite
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

        let world = app.world_mut();

        // Add default sprite asset
        let mut sprites = world.get_resource_mut::<Assets<Sprite>>().unwrap();
        sprites.set_untracked(Handle::<Sprite>::default(), Sprite::default());

        // Set the pipeline and rendering
        let mut render_graph = world.get_resource_mut::<RenderGraph>().unwrap();

        render_graph.add_system_node(
            node::SPRITE_INSTANCE,
            RenderResourcesNode::<SpriteInstance>::new(true),
        );
        render_graph
            .add_node_edge(node::SPRITE_INSTANCE, base::node::MAIN_PASS)
            .unwrap();

        render_graph.add_system_node(node::SPRITE, AssetRenderResourcesNode::<Sprite>::new(true));
        render_graph
            .add_node_edge(node::SPRITE, base::node::MAIN_PASS)
            .unwrap();

        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let pipeline = build_sprite_pipeline(&mut shaders);

        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        pipelines.set_untracked(SPRITE_PIPELINE_HANDLE, pipeline);
    }
}
