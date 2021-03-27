use std::ops::Mul;

use bevy::prelude::*;

use super::WorldToLocal;

/// 2D analogue of [`WorldToLocal`](super::WorldToLocal)
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct WorldToLocal2D(pub Mat3);

impl Default for WorldToLocal2D {
    #[inline]
    fn default() -> Self {
        Self(Mat3::IDENTITY)
    }
}

impl Mul for WorldToLocal2D {
    type Output = WorldToLocal2D;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        WorldToLocal2D(self.0.mul_mat3(&rhs.0))
    }
}

impl From<WorldToLocal> for WorldToLocal2D {
    #[inline]
    fn from(value: WorldToLocal) -> Self {
        WorldToLocal2D(Mat3::from_cols(
            Vec2::from(value.0.x_axis).extend(0.0),
            Vec2::from(value.0.y_axis).extend(0.0),
            Vec2::from(value.0.w_axis).extend(1.0),
        ))
    }
}

impl From<Mat3> for WorldToLocal2D {
    #[inline]
    fn from(value: Mat3) -> Self {
        Self(value)
    }
}
