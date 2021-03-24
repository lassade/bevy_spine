use bevy::prelude::*;

pub use super::{ChildOfTransform2D, RootTransform2D, Transform2D};

pub fn tagging_system(
    commands: &mut Commands,
    root_query: Query<Entity, (Without<Parent>, Without<RootTransform2D>, With<Transform2D>)>,
    parent_query: Query<(Option<&Transform>, Option<&Transform2D>)>,
    // TODO: Test perf with the queries bellow
    // parent_query: Query<(), (Without<Transform>, With<Transform2D>)>,
    // parent_query: Query<(), (With<Transform>, Without<Transform2D>)>,
    child_query: Query<Entity, (Changed<Parent>, With<Transform2D>)>,
    mixed_child_query: Query<Entity, (Changed<Parent>, With<Transform>)>,
) {
    for entity in root_query.iter() {
        commands.insert_one(entity, RootTransform2D);
    }

    for entity in child_query.iter() {
        match parent_query.get(entity) {
            Ok((Some(_), None)) => {
                // `Transform` parent
                commands.insert_one(entity, RootTransform2D);
            }
            Ok((None, Some(_))) => {
                // `Transform2D` parent
                commands.remove_one::<RootTransform2D>(entity);
            }
            _ => {}
        }
    }

    for entity in mixed_child_query.iter() {
        match parent_query.get(entity) {
            Ok((Some(_), None)) => {
                // `Transform` parent
                commands.remove_one::<ChildOfTransform2D>(entity);
            }
            Ok((None, Some(_))) => {
                // `Transform2D` parent
                commands.insert_one(entity, ChildOfTransform2D);
            }
            _ => {}
        }
    }
}
