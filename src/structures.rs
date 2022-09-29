use std::ops::{Add, Sub, Mul, DivAssign};

use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Tri {
    pub p: [Vec3; 3]
}

// #[derive(Clone)]
pub struct Mesh {
    pub tris: Vec<Tri>
}

pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4]
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn with(x: f32, y: f32, z: f32) -> Self {
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

    pub fn into_point(&self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
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

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self { x: self.x * other, y: self.y * other, z: self.z * other }
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
    pub fn new() -> Self {
        Self {
            p: [
                Vec3::new(),
                Vec3::new(),
                Vec3::new()
            ]
        }
    }

    // fn from_vec3s(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {
    //     Self {
    //         p: [p0, p1, p2]
    //     }
    // }

    pub fn from_points(p0: [f32; 9]) -> Self {
        Self {
            p: [
                Vec3::with(p0[0], p0[1], p0[2]),
                Vec3::with(p0[3], p0[4], p0[5]),
                Vec3::with(p0[6], p0[7], p0[8])
            ]
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas.draw_line(self.p[0].into_point(), self.p[1].into_point()).expect("Failed to draw line");
        canvas.draw_line(self.p[1].into_point(), self.p[2].into_point()).expect("Failed to draw line");
        canvas.draw_line(self.p[2].into_point(), self.p[0].into_point()).expect("Failed to draw line");
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
                result.m[r][c] = self.m[r][0] * other.m[0][c] + self.m[r][1] * other.m[1][c] + self.m[r][2] * other.m[2][c] + self.m[r][3] * other.m[3][c];
            }
        }

        result
    }
}
