use std::ops::Mul;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::Color;

use sdl2;
use sdl2::{
    video::Window,
    render::Canvas,
    EventPump
};

struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

struct Tri {
    p: [Vec3; 3]
}

// #[derive(Clone)]
struct Mesh {
    tris: Vec<Tri>
}


impl Vec3 {
    fn new(arr: [f32; 3]) -> Self {
        Self { x: arr[0], y: arr[1], z: arr[2] }
    }

    fn project(&self, m: &Matrix4x4, p_out: &mut Self){
        p_out.x = self.x * m.m[0][0] + self.y * m.m[1][0] + self.z * m.m[2][0] + m.m[3][0];
        p_out.y = self.x * m.m[0][1] + self.y * m.m[1][1] + self.z * m.m[2][1] + m.m[3][1];
        p_out.z = self.x * m.m[0][2] + self.y * m.m[1][2] + self.z * m.m[2][2] + m.m[3][2];
        let w = self.x * m.m[0][3] + self.y * m.m[1][3] + self.z * m.m[2][3] + m.m[3][3];

        if w != 0.0 {
            p_out.x /= w;
            p_out.y /= w;
            p_out.z /= w;
        }
    }

    fn into_point(&self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
    }

    fn add(&self, other: &Vec3) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Tri {
    fn new() -> Self {
        Self {
            p: [
                Vec3::new([0.0, 0.0, 0.0]),
                Vec3::new([0.0, 0.0, 0.0]),
                Vec3::new([0.0, 0.0, 0.0])
            ]
        }
    }

    // fn from_vec3s(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {
    //     Self {
    //         p: [p0, p1, p2]
    //     }
    // }

    fn from_points(p0: [f32; 9]) -> Self {
        Self {
            p: [
                Vec3::new([p0[0], p0[1], p0[2]]),
                Vec3::new([p0[3], p0[4], p0[5]]),
                Vec3::new([p0[6], p0[7], p0[8]])
            ]
        }
    }

    // Unsafe drawing function
    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas.draw_line(self.p[0].into_point(), self.p[1].into_point()).expect("Failed to draw line");
        canvas.draw_line(self.p[1].into_point(), self.p[2].into_point()).expect("Failed to draw line");
        canvas.draw_line(self.p[2].into_point(), self.p[0].into_point()).expect("Failed to draw line");
    }
}

impl Mesh {
    fn new(tris: Vec<Tri>) -> Self {
        Self { tris }
    }
}


struct Matrix4x4 {
    m: [[f32; 4]; 4]
}

impl Matrix4x4 {
    fn new() -> Self {
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

// Window size
const WIDTH: usize = 800;
const HEIGHT: usize = 600;


/// Application entry point
pub fn main() {
    // Create a new window
    let (mut canvas, mut event_pump) = init_sdl();

    let z_near = 0.1; // Near clipping plane
    let z_far = 1000.0; // Far clipping plane
    let a = WIDTH as f32/HEIGHT as f32; // Aspect ratio
    let fov = 90.0; // Field of view
    let fov = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();

    let cube_mesh = Mesh::new(vec![
        // SOUTH
        Tri::from_points([-1.0, -1.0, -1.0,-1.0,  1.0, -1.0, 1.0,  1.0, -1.0]),
        Tri::from_points([-1.0, -1.0, -1.0, 1.0,  1.0, -1.0, 1.0, -1.0, -1.0]),
        // EAST
        Tri::from_points([ 1.0, -1.0, -1.0, 1.0,  1.0, -1.0, 1.0,  1.0,  1.0]),
        Tri::from_points([ 1.0, -1.0, -1.0, 1.0,  1.0,  1.0, 1.0, -1.0,  1.0]),
        // NORTH
        Tri::from_points([ 1.0, -1.0,  1.0, 1.0,  1.0,  1.0,-1.0,  1.0,  1.0]),
        Tri::from_points([ 1.0, -1.0,  1.0,-1.0,  1.0,  1.0,-1.0, -1.0,  1.0]),
        // WEST
        Tri::from_points([-1.0, -1.0,  1.0,-1.0,  1.0,  1.0,-1.0,  1.0, -1.0]),
        Tri::from_points([-1.0, -1.0,  1.0,-1.0,  1.0, -1.0,-1.0, -1.0, -1.0]),
        // TOP
        Tri::from_points([-1.0,  1.0, -1.0,-1.0,  1.0,  1.0, 1.0,  1.0,  1.0]),
        Tri::from_points([-1.0,  1.0, -1.0, 1.0,  1.0,  1.0, 1.0,  1.0, -1.0]),
        // BOTTOM
        Tri::from_points([-1.0, -1.0,  1.0,-1.0, -1.0, -1.0, 1.0, -1.0, -1.0]),
        Tri::from_points([-1.0, -1.0,  1.0, 1.0, -1.0, -1.0, 1.0, -1.0,  1.0])
    ]);
    // Projection matrix
    let mut proj_matrix: Matrix4x4;

    // Rotation matrix
    let mut theta:f32 = 0.0;
    let mut rot_matrix_z: Matrix4x4;
    let mut rot_matrix_x: Matrix4x4;
    let mut rot_matrix: Matrix4x4;


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(30, 34, 34));
        canvas.clear();

        // Update Matrices
        proj_matrix = Matrix4x4 {
            m: [
                [fov/a, 0.0, 0.0, 0.0],
                [0.0, fov, 0.0, 0.0],
                [0.0, 0.0, z_far/(z_far-z_near), 1.0],
                [0.0, 0.0, (-z_far*z_near)/(z_far-z_near), 0.0]
            ]
        };

        // Ripped from Wikipedia
        theta += 0.02;
        rot_matrix_z = Matrix4x4 { m:[
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]};
        rot_matrix_x = Matrix4x4 { m:[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, theta.cos(), theta.sin(), 0.0],
            [0.0, -theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]};

        // Multiply rotation matrices
        rot_matrix = rot_matrix_z * rot_matrix_x;

        // Draw Triangles
        let mut rotated: Tri = Tri::new();
        let mut translated:Tri = Tri::from_points([0.0; 9]);
        let mut projected = Tri::from_points([0.0; 9]);
        for triangle in cube_mesh.tris.iter() {
            // Rotate
            for i in 0..3 {
                triangle.p[i].project(&rot_matrix, &mut rotated.p[i]);
            }

            // Translate
            translated.p[0] = rotated.p[0].add(&Vec3::new([0.0, 0.0, 5.0]));
            translated.p[1] = rotated.p[1].add(&Vec3::new([0.0, 0.0, 5.0]));
            translated.p[2] = rotated.p[2].add(&Vec3::new([0.0, 0.0, 5.0]));
            // Project triangles from 3D to 2D
            for i in 0..3 {
                translated.p[i].project(&proj_matrix, &mut projected.p[i]);
            }
            // Scale into view
            for i in 0..3 {
                projected.p[i].x += 1.0;
                projected.p[i].y += 1.0;
                projected.p[i].x *= 0.5 * WIDTH as f32;
                projected.p[i].y *= 0.5 * HEIGHT as f32;
            }

            // Draw triangles to screen
            projected.draw(&mut canvas, Color::RGB(255, 255, 255));

        }


        // canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


/// Initialize SDL
fn init_sdl() -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Gráficos Chingones", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump)
}