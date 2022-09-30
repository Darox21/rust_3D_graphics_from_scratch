use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fs;
use std::io::Read;

use sdl2::gfx::primitives::DrawRenderer;


use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Point;

/// A 3D vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// A 3D triangle, with 3 vertices
/// and an optional color
#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub p: [Vec3; 3],
    pub c: Option<Color>
}

/// A Vector of 3D triangles
pub struct Mesh {
    pub tris: Vec<Tri>
}

/// A 4x4 matrix for 3D transformations
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


    /// Multiplies the vector by a 4x4 matrix, returning a new vector
    /// The extra 4th dimension is assumed is used to scale the vector
    pub fn dot_by_matrix(&self, m: &Matrix4x4, p_out: &mut Self){
        p_out.x = self.x * m.m[0][0] + self.y * m.m[1][0] + self.z * m.m[2][0] + m.m[3][0];
        p_out.y = self.x * m.m[0][1] + self.y * m.m[1][1] + self.z * m.m[2][1] + m.m[3][1];
        p_out.z = self.x * m.m[0][2] + self.y * m.m[1][2] + self.z * m.m[2][2] + m.m[3][2];
        let w = self.x * m.m[0][3] + self.y * m.m[1][3] + self.z * m.m[2][3] + m.m[3][3];

        if w != 0.0 {
            *p_out /= w;
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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

    pub fn draw(&self, canvas: &mut Canvas<Window>, outline_color: Color) {
        canvas.set_draw_color(outline_color);
        canvas.aa_trigon(
            self.p[0].x as i16, self.p[0].y as i16,
            self.p[1].x as i16, self.p[1].y as i16,
            self.p[2].x as i16, self.p[2].y as i16,
            outline_color
        ).unwrap();
    }

    pub fn draw_filled(&self, canvas: &mut Canvas<Window>) {
        let color = self.c.unwrap_or(Color::WHITE);
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
                Vec3::from([p[6], p[7], p[8]]),
            ],
            c: None
        }
    }
}

impl From<[Vec3; 3]> for Tri {
    fn from(p: [Vec3; 3]) -> Self {
        Self {
            p: [p[0], p[1], p[2]],
            c: None
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
            ],
            c: self.c
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
            ],
            c: self.c
        }
    }
}


impl Mesh {
    pub fn new(tris: Vec<Tri>) -> Self {
        Self { tris }
    }

    pub fn load_from_file(filename: &str) -> Self {
        let mut tris = Vec::new();
        let mut tri: [Vec3; 3];
        let mut vertex: Vec3;
        let mut vertex_buffer: Vec<Vec3> = Vec::new();
        let mut relationships: [Vec<i32>;3];
        let mut line_elements: Vec<&str>;


        let mut file = fs::File::open(filename).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let lines: Vec<&str> = contents.split("\n").collect();
        for line in lines {
            line_elements = line.split(" ").map(|s| s.trim()).collect();
            match line_elements[..] {
                ["v", x, y, z, ..] => {
                    // println!("Vertex: {}, {}, {}", x, y, z);
                    vertex = Vec3::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
                    vertex_buffer.push(vertex);
                },
                ["f", v1, v2, v3, ..] => {
                    // println!("Face: {}, {}, {}", v1, v2, v3);
                    relationships = [
                        v1.split("/").map(|s| s.parse().unwrap()).collect(),
                        v2.split("/").map(|s| s.parse().unwrap()).collect(),
                        v3.split("/").map(|s| s.parse().unwrap()).collect()
                    ];
                    tri = [
                        vertex_buffer[relationships[0][0] as usize - 1],
                        vertex_buffer[relationships[1][0] as usize - 1],
                        vertex_buffer[relationships[2][0] as usize - 1]
                    ];
                    tris.push(Tri::from(tri));
                },
                _ => ()
            }
        }
        Self::new(tris)
    }

    pub fn sort(&mut self) {
        self.tris.sort_by(|a, b| {
            let dist_a = (a.p[0].z + a.p[1].z + a.p[2].z) / 3.0;
            let dist_b = (b.p[0].z + b.p[1].z + b.p[2].z) / 3.0;
            dist_b.partial_cmp(&dist_a).unwrap()
        });
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