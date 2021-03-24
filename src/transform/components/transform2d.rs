use bevy::{math::Mat2, prelude::*};

// TODO: Tag components to mix and match 2d and 3d nodes

/// Tags the top most [`Transform2D`] entity in the hierarchy group,
/// meaning that it can also be a child of [`Transform`] and still be considered root;
///
/// Used by [`local_to_world_2d5_system`](super::super::systems::local_to_world_2d5_system) of the 2.5D environment.
#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct RootTransform2D;

/// Tags any [`Transform`] entity that is a child of a [`Transform2D`];
///
/// Used by [`local_to_world_children_of_transform_2d_system`](super::super::systems::local_to_world_children_of_transform_2d_system) of the 2.5D environment;
#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct ChildOfTransform2D;

/// 2D analogue of a [`Transform`]
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Transform2D {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub shear: Vec2,
}

impl Default for Transform2D {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform2D {
    /// Create a new [`Transform2D`] at the position `(x, y, z)`
    #[inline]
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self::from_translation(Vec2::new(x, y))
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            shear: Vec2::ZERO,
        }
    }

    // TODO
    // #[inline]
    // pub fn from_matrix(matrix: Mat3) -> Self {
    //     let (scale, rotation, translation) = matrix.to_scale_rotation_translation();

    //     Transform2D {
    //         translation,
    //         rotation,
    //         scale,
    //     }
    // }

    #[inline]
    pub fn from_translation(translation: Vec2) -> Self {
        Transform2D {
            translation,
            ..Default::default()
        }
    }

    #[inline]
    pub fn from_rotation(rotation: f32) -> Self {
        Transform2D {
            rotation,
            ..Default::default()
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec2) -> Self {
        Transform2D {
            scale,
            ..Default::default()
        }
    }

    // TODO:
    // /// Returns transform with the same translation and scale, but rotation so that transform.forward() points at target
    // #[inline]
    // pub fn looking_at(mut self, target: Vec2, up: Vec2) -> Self {
    //     self.look_at(target, up);
    //     self
    // }

    #[inline]
    pub fn compute_matrix(&self) -> Mat3 {
        let m = Mat2::from_cols(Vec2::new(1.0, self.shear.x), Vec2::new(self.shear.y, 1.0))
            .mul_mat2(&Mat2::from_scale_angle(self.scale, self.rotation));
        Mat3::from_cols(
            m.x_axis.extend(0.0),
            m.x_axis.extend(0.0),
            self.translation.extend(1.0),
        )
    }

    // TODO:
    // #[inline]
    // /// Get the unit vector in the local x direction
    // pub fn local_x(&self) -> Vec3 {
    //     self.rotation * Vec3::unit_x()
    // }

    // #[inline]
    // /// Get the unit vector in the local y direction
    // pub fn local_y(&self) -> Vec3 {
    //     self.rotation * Vec3::unit_y()
    // }

    // #[inline]
    // /// Get the unit vector in the local z direction
    // pub fn local_z(&self) -> Vec3 {
    //     self.rotation * Vec3::unit_z()
    // }

    #[inline]
    /// Rotate the transform by the given rotation
    pub fn rotate(&mut self, rotation: f32) {
        self.rotation += rotation;
    }

    // TODO:
    // #[inline]
    // pub fn mul_transform(&self, transform: Transform) -> GlobalTransform {
    //     let translation = self.mul_vec3(transform.translation);
    //     let rotation = self.rotation * transform.rotation;
    //     let scale = self.scale * transform.scale;
    //     GlobalTransform {
    //         scale,
    //         rotation,
    //         translation,
    //     }
    // }

    // #[inline]
    // pub fn mul_vec3(&self, mut value: Vec3) -> Vec3 {
    //     value = self.rotation * value;
    //     value = self.scale * value;
    //     value += self.translation;
    //     value
    // }

    // #[inline]
    // pub fn apply_non_uniform_scale(&mut self, scale: Vec3) {
    //     self.scale *= scale;
    // }

    // #[inline]
    // pub fn look_at(&mut self, target: Vec3, up: Vec3) {
    //     let forward = Vec3::normalize(self.translation - target);
    //     let right = up.cross(forward).normalize();
    //     let up = forward.cross(right);
    //     self.rotation = Quat::from_rotation_mat3(&Mat3::from_cols(right, up, forward));
    // }
}
