use bevy::prelude::*;

use super::{ChildOfTransform2D, RootTransform2D, Transform2D};

pub fn transform_tagging_system(
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

#[cfg(test)]
mod tests {
    #[test]
    fn transform_2d_has_no_parent_but_is_tagged_as_root() {
        todo!()
    }

    #[test]
    fn transform_2d_is_child_of_a_transform_and_tagged_as_root() {
        todo!()
    }

    #[test]
    fn transform_is_child_of_a_transform_2d_and_tagged_as_such() {
        todo!()
    }
}
