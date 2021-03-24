use bevy::{
    prelude::*,
    render::render_graph::{base, RenderGraph, RenderResourcesNode},
    transform::TransformSystem,
};

mod components;
mod local_to_world_2d5_system;
mod local_to_world_2d_system;
mod local_to_world_children_of_transform_2d_system;
mod local_to_world_system;
mod tagging_system;

pub use components::*;

pub mod node {
    pub const LOCAL_TO_WORLD: &str = "local_to_world";
    pub const LOCAL_TO_WORLD_2D: &str = "local_to_world_2d";
}

pub mod systems {
    pub use super::local_to_world_2d5_system::*;
    pub use super::local_to_world_2d_system::*;
    pub use super::local_to_world_children_of_transform_2d_system::*;
    pub use super::local_to_world_system::*;
    pub use super::tagging_system::*;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Transform2D5System {
    Tagging,
    PropagateTransform,
    PropagateTransform2D,
    ChildOfTransform2DPropagate,
}

#[derive(Default)]
pub struct Transform2DPlugin;

impl Plugin for Transform2DPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<LocalToWorld2D>()
            .register_type::<Transform2D>();

        // Transform
        app.add_startup_system_to_stage(
            StartupStage::PostStartup,
            systems::local_to_world_2d_system
                .system()
                .label(Transform2D5System::PropagateTransform2D)
                .after(TransformSystem::ParentUpdate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            systems::local_to_world_2d_system
                .system()
                .label(Transform2D5System::PropagateTransform2D)
                .after(TransformSystem::ParentUpdate),
        );

        let resources = app.resources_mut();

        // Set the pipeline and rendering
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        // Custom transform matrix
        render_graph.add_system_node(
            node::LOCAL_TO_WORLD_2D,
            RenderResourcesNode::<LocalToWorld2D>::new(true),
        );
        render_graph
            .add_node_edge(node::LOCAL_TO_WORLD_2D, base::node::MAIN_PASS)
            .unwrap();
    }
}

/// Registers the components [`LocalToWorld`], [`Transform2D`],
/// [`ChildOfTransform2D`], [`RootTransform2D`] and the system needed to use them;
///
///
#[derive(Default)]
pub struct Transform2D5Plugin;

impl Plugin for Transform2D5Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<LocalToWorld>()
            .register_type::<Transform2D>()
            .register_type::<ChildOfTransform2D>()
            .register_type::<RootTransform2D>();

        // Transform
        app.add_startup_system_to_stage(
            StartupStage::PostStartup,
            systems::tagging_system
                .system()
                .label(Transform2D5System::Tagging)
                .after(TransformSystem::ParentUpdate),
        )
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            systems::local_to_world_system
                .system()
                .label(Transform2D5System::PropagateTransform)
                .after(TransformSystem::ParentUpdate),
        )
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            systems::local_to_world_2d5_system
                .system()
                .label(Transform2D5System::PropagateTransform2D)
                .after(Transform2D5System::PropagateTransform),
        )
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            systems::local_to_world_children_of_transform_2d_system
                .system()
                .label(Transform2D5System::ChildOfTransform2DPropagate)
                .after(Transform2D5System::PropagateTransform2D),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            systems::tagging_system
                .system()
                .label(Transform2D5System::Tagging)
                .after(TransformSystem::ParentUpdate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            systems::local_to_world_system
                .system()
                .label(Transform2D5System::PropagateTransform)
                .after(TransformSystem::ParentUpdate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            systems::local_to_world_2d5_system
                .system()
                .label(Transform2D5System::PropagateTransform2D)
                .after(Transform2D5System::PropagateTransform),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            systems::local_to_world_children_of_transform_2d_system
                .system()
                .label(Transform2D5System::ChildOfTransform2DPropagate)
                .after(Transform2D5System::PropagateTransform2D),
        );

        let resources = app.resources_mut();

        // Set the pipeline and rendering
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        // Custom transform matrix
        render_graph.add_system_node(
            node::LOCAL_TO_WORLD,
            RenderResourcesNode::<LocalToWorld>::new(true),
        );
        render_graph
            .add_node_edge(node::LOCAL_TO_WORLD, base::node::MAIN_PASS)
            .unwrap();
    }
}

/// Registers the component [`LocalToWorld`] and the system needed to use them
#[derive(Default)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<LocalToWorld>();

        // Transform
        app.add_startup_system_to_stage(
            StartupStage::PostStartup,
            systems::local_to_world_system
                .system()
                .label(Transform2D5System::PropagateTransform)
                .after(TransformSystem::ParentUpdate),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            systems::local_to_world_system
                .system()
                .label(Transform2D5System::PropagateTransform)
                .after(TransformSystem::ParentUpdate),
        );

        let resources = app.resources_mut();

        // Set the pipeline and rendering
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        // Custom transform matrix
        render_graph.add_system_node(
            node::LOCAL_TO_WORLD,
            RenderResourcesNode::<LocalToWorld>::new(true),
        );
        render_graph
            .add_node_edge(node::LOCAL_TO_WORLD, base::node::MAIN_PASS)
            .unwrap();
    }
}
