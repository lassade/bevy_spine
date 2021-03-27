// flags
pub enum TransformPropagationConstraint {
    None,
    Translation,
    Rotation,
    Scale,
    Reflection,
}

// - normal
// Do nothing
//
// - onlyTranslation
// mat.x_axis = [1, 0, 0, 0];
// mat.y_axis = [0, 1, 0, 0];
// mat.z_axis = [0, 0, 1, 0];
//
// - noRotationOrReflection
// mat.x_axis = [mat.x_axis.length(), 0, 0, 0];
// mat.y_axis = [0, mat.y_axis.length(), 0, 0];
// mat.z_axis = [0, 0, mat.z_axis.length(), 0];
//
// - noScale
// mat.x_axis = mat.x_axis.normalized();
// mat.y_axis = mat.y_axis.normalized();
// mat.z_axis = mat.z_axis.normalized();
//
// - noScaleOrReflection
//
//
