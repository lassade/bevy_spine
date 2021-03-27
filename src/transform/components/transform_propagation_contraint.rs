use bevy::prelude::*;

pub enum TransformPropagationConstraint {
    None,
    OnlyTranslation,
    NoRotationOrReflection,
    NoScale,
    NoScaleOrReflection,
}

impl TransformPropagationConstraint {
    pub fn constrain(&self, transform: &mut Mat4) {
        use TransformPropagationConstraint::*;
        match self {
            None => {}
            OnlyTranslation => {
                transform.x_axis = Vec4::new(1.0, 0.0, 0.0, 0.0);
                transform.y_axis = Vec4::new(0.0, 1.0, 0.0, 0.0);
                transform.z_axis = Vec4::new(0.0, 0.0, 1.0, 0.0);
            }
            NoRotationOrReflection => {
                let det = transform.determinant().signum();
                transform.x_axis = Vec4::new(transform.x_axis.length() * det, 0.0, 0.0, 0.0);
                transform.y_axis = Vec4::new(0.0, transform.y_axis.length(), 0.0, 0.0);
                transform.z_axis = Vec4::new(0.0, 0.0, transform.z_axis.length(), 0.0);
            }
            NoScale => {
                transform.x_axis = transform
                    .x_axis
                    .try_normalize()
                    .unwrap_or(Vec4::new(1.0, 0.0, 0.0, 0.0));
                transform.y_axis = transform
                    .y_axis
                    .try_normalize()
                    .unwrap_or(Vec4::new(0.0, 1.0, 0.0, 0.0));
                transform.z_axis = transform
                    .z_axis
                    .try_normalize()
                    .unwrap_or(Vec4::new(0.0, 0.0, 1.0, 0.0));
            }
            NoScaleOrReflection => {
                let det = transform.determinant();

                // Scale mixed with the rotation and can't be fully extracted
                let scale = Vec3::new(
                    transform.x_axis.length_recip(),
                    transform.y_axis.length_recip(),
                    transform.z_axis.length_recip(),
                );

                if scale.x.is_nan() {
                    transform.x_axis = Vec4::ZERO;
                } else {
                    transform.x_axis = transform.x_axis * scale.x * det.signum();
                }

                if scale.y.is_nan() {
                    transform.y_axis = Vec4::ZERO;
                } else {
                    transform.y_axis = transform.y_axis * scale.y;
                }

                if scale.z.is_nan() {
                    transform.z_axis = Vec4::ZERO;
                } else {
                    transform.z_axis = transform.z_axis * scale.z;
                }
            }
        }
    }

    pub fn constrain_2d(&self, transform: &mut Mat3) {
        use TransformPropagationConstraint::*;
        match self {
            None => {}
            OnlyTranslation => {
                transform.x_axis = Vec3::new(1.0, 0.0, 0.0);
                transform.y_axis = Vec3::new(0.0, 1.0, 0.0);
            }
            NoRotationOrReflection => {
                let det = transform.determinant().signum();
                transform.x_axis = Vec3::new(transform.x_axis.length() * det, 0.0, 0.0);
                transform.y_axis = Vec3::new(0.0, transform.y_axis.length(), 0.0);
            }
            NoScale => {
                transform.x_axis = transform
                    .x_axis
                    .try_normalize()
                    .unwrap_or(Vec3::new(1.0, 0.0, 0.0));
                transform.y_axis = transform
                    .y_axis
                    .try_normalize()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
            }
            NoScaleOrReflection => {
                let det = transform.determinant();

                // Scale mixed with the rotation and can't be fully extracted
                let scale = Vec2::new(
                    transform.x_axis.length_recip(),
                    transform.y_axis.length_recip(),
                );

                if scale.x.is_nan() {
                    transform.x_axis = Vec3::ZERO;
                } else {
                    transform.x_axis = transform.x_axis * scale.x * det.signum();
                }

                if scale.y.is_nan() {
                    transform.y_axis = Vec3::ZERO;
                } else {
                    transform.y_axis = transform.y_axis * scale.y;
                }
            }
        }
    }
}
