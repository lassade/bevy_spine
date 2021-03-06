use bevy::prelude::*;

use super::{ChildOfTransform2D, LocalToWorld, Transform2D};

// TODO: DontPropagateTransform
// TODO: Because of the tagging system these objects may have a 1 frame delay to update

/// Propagate transform for [`Transform`]s that are children of [`Transform2D`](super::Transform2D)
pub fn local_to_world_children_of_transform_2d_system(
    mut root_query: Query<
        (Entity, Option<&Children>, &Transform, &mut LocalToWorld),
        (
            With<ChildOfTransform2D>,
            With<LocalToWorld>,
            Without<Transform2D>,
        ),
    >,
    mut transform_query: Query<
        (&Transform, &mut LocalToWorld),
        (
            Without<ChildOfTransform2D>,
            With<Parent>,
            Without<Transform2D>,
        ),
    >,
    parent_query: Query<&LocalToWorld, With<Transform2D>>,
    changed_transform_query: Query<Entity, Changed<Transform>>,
    children_query: Query<Option<&Children>, (With<Parent>, With<LocalToWorld>)>,
) {
    for (entity, children, transform, mut global_transform) in root_query.iter_mut() {
        let mut changed = false;
        if changed_transform_query.get(entity).is_ok() {
            *global_transform = LocalToWorld::from(*transform);

            if let Ok(parent) = parent_query.get(entity) {
                *global_transform = (*parent) * (*global_transform);
            }

            changed = true;
        }

        if let Some(children) = children {
            for child in children.iter() {
                propagate_recursive(
                    &global_transform,
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

fn propagate_recursive(
    parent: &LocalToWorld,
    changed_transform_query: &Query<Entity, Changed<Transform>>,
    transform_query: &mut Query<
        (&Transform, &mut LocalToWorld),
        (
            Without<ChildOfTransform2D>,
            With<Parent>,
            Without<Transform2D>,
        ),
    >,
    children_query: &Query<Option<&Children>, (With<Parent>, With<LocalToWorld>)>,
    entity: Entity,
    mut changed: bool,
) {
    changed |= changed_transform_query.get(entity).is_ok();

    let global_matrix = {
        if let Ok((transform, mut global_transform)) = transform_query.get_mut(entity) {
            if changed {
                *global_transform = (*parent) * LocalToWorld::from(*transform);
            }
            *global_transform
        } else {
            return;
        }
    };

    if let Ok(Some(children)) = children_query.get(entity) {
        for child in children.iter() {
            propagate_recursive(
                &global_matrix,
                changed_transform_query,
                transform_query,
                children_query,
                *child,
                changed,
            );
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     // use crate::hierarchy::{parent_update_system, BuildChildren, BuildWorldChildren};
//     // use bevy_ecs::{Resources, Schedule, Stage, SystemStage, World};

//     #[test]
//     fn did_propagate() {
//         let mut world = World::default();
//         let mut resources = Resources::default();

//         let mut update_stage = SystemStage::parallel();
//         update_stage.add_system(parent_update_system.system());
//         update_stage.add_system(transform_propagate_system.system());

//         let mut schedule = Schedule::default();
//         schedule.add_stage("update", update_stage);

//         // Root entity
//         world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), LocalToWorld::identity()));

//         let mut children = Vec::new();
//         world
//             .build()
//             .spawn((Transform::from_xyz(1.0, 0.0, 0.0), LocalToWorld::identity()))
//             .with_children(|parent| {
//                 parent
//                     .spawn((Transform::from_xyz(0.0, 2.0, 0.), LocalToWorld::identity()))
//                     .for_current_entity(|entity| children.push(entity))
//                     .spawn((Transform::from_xyz(0.0, 0.0, 3.), LocalToWorld::identity()))
//                     .for_current_entity(|entity| children.push(entity));
//             });
//         schedule.run(&mut world, &mut resources);

//         assert_eq!(
//             *world.get::<LocalToWorld>(children[0]).unwrap(),
//             LocalToWorld::from_xyz(1.0, 0.0, 0.0) * Transform::from_xyz(0.0, 2.0, 0.0)
//         );

//         assert_eq!(
//             *world.get::<LocalToWorld>(children[1]).unwrap(),
//             LocalToWorld::from_xyz(1.0, 0.0, 0.0) * Transform::from_xyz(0.0, 0.0, 3.0)
//         );
//     }

//     #[test]
//     fn did_propagate_command_buffer() {
//         let mut world = World::default();
//         let mut resources = Resources::default();

//         let mut update_stage = SystemStage::parallel();
//         update_stage.add_system(parent_update_system.system());
//         update_stage.add_system(transform_propagate_system.system());

//         let mut schedule = Schedule::default();
//         schedule.add_stage("update", update_stage);

//         // Root entity
//         let mut commands = Commands::default();
//         commands.set_entity_reserver(world.get_entity_reserver());
//         let mut children = Vec::new();
//         commands
//             .spawn((Transform::from_xyz(1.0, 0.0, 0.0), LocalToWorld::identity()))
//             .with_children(|parent| {
//                 parent
//                     .spawn((Transform::from_xyz(0.0, 2.0, 0.0), LocalToWorld::identity()))
//                     .for_current_entity(|entity| children.push(entity))
//                     .spawn((Transform::from_xyz(0.0, 0.0, 3.0), LocalToWorld::identity()))
//                     .for_current_entity(|entity| children.push(entity));
//             });
//         commands.apply(&mut world, &mut resources);
//         schedule.run(&mut world, &mut resources);

//         assert_eq!(
//             *world.get::<LocalToWorld>(children[0]).unwrap(),
//             LocalToWorld::from_xyz(1.0, 0.0, 0.0) * Transform::from_xyz(0.0, 2.0, 0.0)
//         );

//         assert_eq!(
//             *world.get::<LocalToWorld>(children[1]).unwrap(),
//             LocalToWorld::from_xyz(1.0, 0.0, 0.0) * Transform::from_xyz(0.0, 0.0, 3.0)
//         );
//     }
// }
