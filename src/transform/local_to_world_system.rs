use bevy::prelude::*;

use super::components::{
    ChildOfTransform2D, DontPropagateTransform, LocalToWorld, Shear, TransformPropagationConstraint,
};

/// Uses the local [`Transform`] to update [`LocalToWorld`] matrices, analogue to the
/// [`transform_propagate_system`](bevy::transform::transform_propagate_system) system function.
pub fn local_to_world_system(
    mut root_query: Query<
        (
            Entity,
            Option<&Children>,
            Option<&DontPropagateTransform>,
            &Transform,
            Option<&Shear>,
            &mut LocalToWorld,
        ),
        (Without<Parent>, With<LocalToWorld>),
    >,
    mut transform_query: Query<
        (
            &Transform,
            Option<&Shear>,
            Option<&DontPropagateTransform>,
            Option<&TransformPropagationConstraint>,
            &mut LocalToWorld,
        ),
        With<Parent>,
    >,
    changed_transform_query: Query<
        Entity,
        Or<(
            Changed<Transform>,
            Added<DontPropagateTransform>,
            Changed<TransformPropagationConstraint>,
        )>,
    >,
    children_query: Query<
        Option<&Children>,
        (
            With<Parent>,
            With<LocalToWorld>,
            Without<ChildOfTransform2D>,
        ),
    >,
) {
    for (entity, children, propagate, transform, shear, mut global_transform) in
        root_query.iter_mut()
    {
        let mut changed = false;
        if changed_transform_query.get(entity).is_ok() {
            *global_transform = LocalToWorld::from(*transform);
            if let Some(shear) = shear {
                global_transform.0 = shear.compute_matrix() * global_transform.0;
            }
            changed = true;
        }

        if propagate.is_some() {
            continue;
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
    changed_transform_query: &Query<
        Entity,
        Or<(
            Changed<Transform>,
            Added<DontPropagateTransform>,
            Changed<TransformPropagationConstraint>,
        )>,
    >,
    transform_query: &mut Query<
        (
            &Transform,
            Option<&Shear>,
            Option<&DontPropagateTransform>,
            Option<&TransformPropagationConstraint>,
            &mut LocalToWorld,
        ),
        With<Parent>,
    >,
    children_query: &Query<
        Option<&Children>,
        (
            With<Parent>,
            With<LocalToWorld>,
            Without<ChildOfTransform2D>,
        ),
    >,
    entity: Entity,
    mut changed: bool,
) {
    changed |= changed_transform_query.get(entity).is_ok();

    let global_matrix = {
        if let Ok((transform, shear, propagate, constraint, mut global_transform)) =
            transform_query.get_mut(entity)
        {
            if changed {
                let mut local_transform = LocalToWorld::from(*transform);

                // Apply shear
                if let Some(shear) = shear {
                    local_transform.0 = shear.compute_matrix() * local_transform.0;
                }

                // Apply propagation constraint
                let mut parent = *parent;
                if let Some(constraint) = constraint {
                    constraint.constrain(&mut parent.0);
                }

                // Calculate the final matrix
                *global_transform = parent * local_transform;
            }

            // Decide if propagate or not
            if propagate.is_some() {
                return;
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
