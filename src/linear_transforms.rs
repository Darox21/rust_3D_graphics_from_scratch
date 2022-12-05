
/*!
# linear_transforms.rs

A Group of functions to perform linear transformations.

Using nalgebra as a linear algebra library.
*/

use nalgebra::{Matrix4, Vector3};

/// Rotation matrix around the X axis
///
/// Output is a 4x4 rotation matrix around the X axis
/// for a given angle in radians.
#[allow(dead_code)]
pub fn rotation_matrix_x(angle: f32) -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, angle.cos(), -angle.sin(), 0.0,
        0.0, angle.sin(), angle.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

/// Rotation matrix around the Y axis
///
/// Output is a 4x4 rotation matrix around the Y axis
/// for a given angle in radians.
#[allow(dead_code)]
pub fn rotation_matrix_y(angle: f32) -> Matrix4<f32> {
    Matrix4::new(
        angle.cos(), 0.0, angle.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -angle.sin(), 0.0, angle.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

/// Rotation matrix around the Z axis
///
/// Output is a 4x4 rotation matrix around the Z axis
/// for a given angle in radians.
#[allow(dead_code)]
pub fn rotation_matrix_z(angle: f32) -> Matrix4<f32> {
    Matrix4::new(
        angle.cos(), -angle.sin(), 0.0, 0.0,
        angle.sin(), angle.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

/// Translation matrix
///
/// Output is a 4x4 translation matrix for a given vector.
pub fn translation_matrix(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    )
}

/// Projection matrix
///
/// * Output is a 4x4 projection matrix
///
/// The near and far planes are used to calculate the z-buffer.
/// The aspect ratio is the ratio of the width to the height of the screen.
/// The FOV is the field of view in radians.
pub fn projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix4<f32> {
    let fov_rad = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();
    Matrix4::new(
        fov_rad / aspect_ratio, 0.0, 0.0, 0.0,
        0.0, fov_rad, 0.0, 0.0,
        0.0, 0.0, far / (far - near), (-far * near) / (far - near),
        0.0, 0.0, 1.0, 0.0
    )
}

/// View matrix
///
/// Output is a 4x4 view matrix for a given position and target. (It is broken)
pub fn view_matrix(camera: Vector3<f32>, view_direction: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let z = (view_direction).normalize();
    let x = up.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    Matrix4::new(
        x.x, y.x, z.x, 0.0,
        x.y, y.y, z.y, 0.0,
        x.z, y.z, z.z, 0.0,
        x.dot(&-camera), y.dot(&-camera), z.dot(&-camera), 1.0
    )
}