use bevy::{
    prelude::*,
    render::render_graph::{base, RenderGraph, RenderResourcesNode},
    transform::TransformSystem,
};

mod components;

pub use components::{GlobalMatrix, ShearTransform};

fn transform_matrix_propagate_system(
    mut root_query: Query<
        (
            Entity,
            Option<&Children>,
            &Transform,
            Option<&ShearTransform>,
            &mut GlobalMatrix,
        ),
        (Without<Parent>, With<GlobalMatrix>),
    >,
    mut transform_query: Query<
        (&Transform, Option<&ShearTransform>, &mut GlobalMatrix),
        With<Parent>,
    >,
    changed_transform_query: Query<Entity, Or<(Changed<Transform>, Changed<ShearTransform>)>>,
    children_query: Query<Option<&Children>, (With<Parent>, With<GlobalMatrix>)>,
) {
    for (entity, children, transform, shear, mut global_transform_matrix) in root_query.iter_mut() {
        let mut changed = false;
        if changed_transform_query.get(entity).is_ok() {
            global_transform_matrix.value = shear
                .map(|shear| shear.compute_matrix(transform))
                .unwrap_or_else(|| transform.compute_matrix());
            changed = true;
        }

        if let Some(children) = children {
            for child in children.iter() {
                propagate_recursive(
                    &global_transform_matrix,
                    &changed_transform_query,
                    &mut transform_query,
                    &children_query,
                    *child,
                    changed,
                );
            }
        }
    }
}

/// Propagate transforms for [`GlobalMatrix`]'s that are children of [`GlobalTransform`]
fn transform_matrix_propagate_from_global_transform_system(
    mut root_query: Query<
        (Option<&Children>, &GlobalTransform),
        (
            Without<Parent>,
            With<GlobalTransform>,
            Changed<GlobalTransform>,
        ),
    >,
    mut transform_query: Query<
        (&Transform, Option<&ShearTransform>, &mut GlobalMatrix),
        With<Parent>,
    >,
    changed_transform_query: Query<Entity, Or<(Changed<Transform>, Changed<ShearTransform>)>>,
    children_query: Query<Option<&Children>, (With<Parent>, With<GlobalMatrix>)>,
) {
    for (children, global_transform) in root_query.iter_mut() {
        if let Some(children) = children {
            for child in children.iter() {
                let parent = GlobalMatrix::from(global_transform.compute_matrix());
                propagate_recursive(
                    &parent,
                    &changed_transform_query,
                    &mut transform_query,
                    &children_query,
                    *child,
                    true,
                );
            }
        }
    }
}

fn propagate_recursive(
    parent: &GlobalMatrix,
    changed_transform_query: &Query<Entity, Or<(Changed<Transform>, Changed<ShearTransform>)>>,
    transform_query: &mut Query<
        (&Transform, Option<&ShearTransform>, &mut GlobalMatrix),
        With<Parent>,
    >,
    children_query: &Query<Option<&Children>, (With<Parent>, With<GlobalMatrix>)>,
    entity: Entity,
    mut changed: bool,
) {
    changed |= changed_transform_query.get(entity).is_ok();

    let global_transform_matrix = {
        if let Ok((transform, shear, mut global_transform_matrix)) = transform_query.get_mut(entity)
        {
            if changed {
                let local_transform_matrix = shear
                    .map(|shear| shear.compute_matrix(transform))
                    .unwrap_or_else(|| transform.compute_matrix());

                global_transform_matrix.value = parent.value.mul_mat4(&local_transform_matrix);
            }
            *global_transform_matrix
        } else {
            return;
        }
    };

    if let Ok(Some(children)) = children_query.get(entity) {
        for child in children.iter() {
            propagate_recursive(
                &global_transform_matrix,
                changed_transform_query,
                transform_query,
                children_query,
                *child,
                changed,
            );
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub mod node {
    pub const GLOBAL_MATRIX: &str = "global_matrix";
}

#[derive(Default)]
pub struct TransformMatrixPlugin;

impl Plugin for TransformMatrixPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Transform
        app.register_type::<GlobalMatrix>()
            .register_type::<ShearTransform>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                transform_matrix_propagate_system
                    .system()
                    .after(TransformSystem::ParentUpdate),
            )
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                transform_matrix_propagate_from_global_transform_system
                    .system()
                    .after(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                transform_matrix_propagate_system
                    .system()
                    .after(TransformSystem::ParentUpdate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                transform_matrix_propagate_from_global_transform_system
                    .system()
                    .after(TransformSystem::TransformPropagate),
            );

        let resources = app.resources_mut();

        // Set the pipeline and rendering
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

        // Custom transform matrix
        render_graph.add_system_node(
            node::GLOBAL_MATRIX,
            RenderResourcesNode::<GlobalMatrix>::new(true),
        );
        render_graph
            .add_node_edge(node::GLOBAL_MATRIX, base::node::MAIN_PASS)
            .unwrap();
    }
}
