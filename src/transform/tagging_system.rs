use bevy::prelude::*;

pub use super::{ChildOfTransform2D, RootTransform2D, Transform2D};

pub fn tagging_system(
    mut commands: Commands,
    root_query: Query<Entity, (Without<Parent>, Without<RootTransform2D>, With<Transform2D>)>,
    parent_query: Query<(Option<&Transform>, Option<&Transform2D>)>,
    child_query: Query<Entity, (Changed<Parent>, With<Transform2D>)>,
    mixed_child_query: Query<Entity, (Changed<Parent>, With<Transform>)>,
) {
    for entity in root_query.iter() {
        commands.entity(entity).insert(RootTransform2D);
    }

    for entity in child_query.iter() {
        match parent_query.get(entity) {
            Ok((Some(_), None)) => {
                // `Transform` parent
                commands.entity(entity).insert(RootTransform2D);
            }
            Ok((None, Some(_))) => {
                // `Transform2D` parent
                commands.entity(entity).remove::<RootTransform2D>();
            }
            _ => {}
        }
    }

    for entity in mixed_child_query.iter() {
        match parent_query.get(entity) {
            Ok((Some(_), None)) => {
                // `Transform` parent
                commands.entity(entity).remove::<ChildOfTransform2D>();
            }
            Ok((None, Some(_))) => {
                // `Transform2D` parent
                commands.entity(entity).insert(ChildOfTransform2D);
            }
            _ => {}
        }
    }
}
