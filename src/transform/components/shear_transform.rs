use bevy::prelude::*;

/// Shear [`Transform`] along each plane, requires a [`GlobalMatrix`] component
///
/// **NOTE** 2D Shearing uses `xy` and `yx` attributes
#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct ShearTransform {
    pub xy: Vec2,
    pub xz: Vec2,
    pub yz: Vec2,
}

impl ShearTransform {
    #[inline]
    pub fn from_shear_xy(x: f32, y: f32) -> Self {
        Self {
            xy: Vec2::new(x, y),
            ..Default::default()
        }
    }

    #[inline]
    pub fn compute_matrix(&self, transform: &Transform) -> Mat4 {
        let shear = Mat4::from_cols(
            Vec4::new(1.0, self.xy.x, self.xz.x, 0.0),
            Vec4::new(self.xy.y, 1.0, self.yz.x, 0.0),
            Vec4::new(self.xz.y, self.yz.y, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        transform.compute_matrix().mul_mat4(&shear)
    }
}
