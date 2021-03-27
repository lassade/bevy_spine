use bevy::prelude::*;

/// Shearing along each plane.
///
/// **NOTE** Shear is a very situational component used to mainly achieve cartoonish motion distortions
/// and should be applied with caution, thats because a sheared [`LocalToWorld`] transform
/// can't be properly converted back to a [`Transform`] transform without causing unwanted
/// distortions.
#[derive(Default, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Shear {
    pub xy: Vec2,
    pub xz: Vec2,
    pub yz: Vec2,
}

impl Shear {
    #[inline]
    pub fn compute_matrix(&self) -> Mat4 {
        Mat4::from_cols(
            Vec4::new(1.0, self.xy.x, self.xz.x, 0.0),
            Vec4::new(self.xy.y, 1.0, self.yz.x, 0.0),
            Vec4::new(self.xz.y, self.yz.y, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}
