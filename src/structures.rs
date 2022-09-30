use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};


use sdl2::gfx::primitives::DrawRenderer;


use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub p: [Vec3; 3]
}

pub struct Mesh {
    pub tris: Vec<Tri>
}

#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn dot_by_matrix(&self, m: &Matrix4x4, p_out: &mut Self){
        p_out.x = self.x * m.m[0][0] + self.y * m.m[1][0] + self.z * m.m[2][0] + m.m[3][0];
        p_out.y = self.x * m.m[0][1] + self.y * m.m[1][1] + self.z * m.m[2][1] + m.m[3][1];
        p_out.z = self.x * m.m[0][2] + self.y * m.m[1][2] + self.z * m.m[2][2] + m.m[3][2];
        let w = self.x * m.m[0][3] + self.y * m.m[1][3] + self.z * m.m[2][3] + m.m[3][3];

        if w != 0.0 {
            *p_out /= w;
        }
    }

    pub fn dot(&self, v: &Self) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }

    pub fn normalize(&mut self) {
        let l = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2]
        }
    }
}

impl From<Vec3> for Point {
    fn from(v: Vec3) -> Self {
        Self::new(v.x as i32, v.y as i32)
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<Matrix4x4> for Vec3 {
    type Output = Self;

    fn mul(self, other: Matrix4x4) -> Self {
        let mut p_out = Self::new(0.0,0.0,0.0);
        self.dot_by_matrix(&other, &mut p_out);
        p_out
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    // Scalar multiplication
    fn mul(self, other: f32) -> Self {
        Self { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    // Scalar division
    fn div(self, other: f32) -> Self {
        Self { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Tri {
    pub fn normal(&self) -> Vec3 {
        let line1 = self.p[1] - self.p[0];
        let line2 = self.p[2] - self.p[0];
        let mut normal = line1.cross(&line2);
        normal.normalize();
        normal
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas.trigon(
            self.p[0].x as i16, self.p[0].y as i16,
            self.p[1].x as i16, self.p[1].y as i16,
            self.p[2].x as i16, self.p[2].y as i16,
            color
        ).unwrap();
    }

    pub fn draw_filled(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas.filled_trigon(
            self.p[0].x as i16, self.p[0].y as i16,
            self.p[1].x as i16, self.p[1].y as i16,
            self.p[2].x as i16, self.p[2].y as i16,
            color
        ).unwrap();
    }
}

impl From<[f32; 9]> for Tri {
    fn from(p: [f32; 9]) -> Self {
        Self {
            p: [
                Vec3::from([p[0], p[1], p[2]]),
                Vec3::from([p[3], p[4], p[5]]),
                Vec3::from([p[6], p[7], p[8]])
            ]
        }
    }
}

impl Add<Vec3> for Tri {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Self {
            p: [
                self.p[0] + other,
                self.p[1] + other,
                self.p[2] + other
            ]
        }
    }
}

impl Mul<Matrix4x4> for Tri {
    type Output = Self;

    fn mul(self, other: Matrix4x4) -> Self {
        Self {
            p: [
                self.p[0] * other,
                self.p[1] * other,
                self.p[2] * other
            ]
        }
    }
}


impl Mesh {
    pub fn new(tris: Vec<Tri>) -> Self {
        Self { tris }
    }
}


impl Matrix4x4 {
    pub fn new() -> Self {
        Self {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Matrix4x4::new();

        for c in 0..4 {
            for r in 0..4 {
                result.m[r][c] =
                    self.m[r][0] * other.m[0][c] +
                    self.m[r][1] * other.m[1][c] +
                    self.m[r][2] * other.m[2][c] +
                    self.m[r][3] * other.m[3][c];
            }
        }

        result
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        v1 += v2;
        assert_eq!(v1, Vec3::new(5.0, 7.0, 9.0));

        v1 -= v2;
        assert_eq!(v1, Vec3::new(1.0, 2.0, 3.0));

        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));

        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(v1.dot(&v2), 32.0);
        assert_eq!(v1.cross(&v2), Vec3::new(-3.0, 6.0, -3.0));
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(v1 * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(v1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_tri() {
        let t1 = Tri::from([ 1.0, -1.0,  1.0, 1.0,  1.0,  1.0,-1.0,  1.0,  1.0]);
        let t2 = Tri::from([-1.0, -1.0,  1.0,-1.0,  1.0,  1.0,-1.0,  1.0, -1.0]);

        assert_eq!(t1.normal(), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(t2.normal(), Vec3::new(-1.0, 0.0, 0.0));
    }
}