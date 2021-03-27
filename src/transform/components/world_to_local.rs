use std::ops::Mul;

use bevy::prelude::*;

use super::WorldToLocal2D;

#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct WorldToLocal(pub Mat4);

impl Default for WorldToLocal {
    #[inline]
    fn default() -> Self {
        Self(Mat4::IDENTITY)
    }
}

impl Mul for WorldToLocal {
    type Output = WorldToLocal;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        WorldToLocal(self.0.mul_mat4(&rhs.0))
    }
}

impl From<WorldToLocal2D> for WorldToLocal {
    #[inline]
    fn from(value: WorldToLocal2D) -> Self {
        WorldToLocal(Mat4::from_cols(
            Vec2::from(value.0.x_axis).extend(0.0).extend(0.0),
            Vec2::from(value.0.y_axis).extend(0.0).extend(0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec2::from(value.0.z_axis).extend(0.0).extend(1.0),
        ))
    }
}

impl From<Mat4> for WorldToLocal {
    #[inline]
    fn from(value: Mat4) -> Self {
        Self(value)
    }
}

// #[cfg(test)]
// mod tests {
//     use bevy::prelude::Transform;
// }
