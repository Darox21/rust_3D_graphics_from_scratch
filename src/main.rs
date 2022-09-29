use std::time::Duration;

mod structures;
use structures::{Vec3, Tri, Mesh, Matrix4x4};

use sdl2;
use sdl2::{
    pixels::Color,
    event::Event,
    video::Window,
    render::Canvas,
    EventPump
};



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

    let mut translation = Vec3::with(0.0, 0.0, 2.0);

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

        translation.z += 0.015;

        // Draw Triangles
        let mut rotated: Tri = Tri::new();
        let mut translated:Tri = Tri::from_points([0.0; 9]);
        let mut projected = Tri::from_points([0.0; 9]);
        let mut normal: Vec3;
        let mut line1: Vec3;
        let mut line2: Vec3;
        for triangle in cube_mesh.tris.iter() {
            // Rotate
            for i in 0..3 {
                triangle.p[i].dot_by_matrix(&rot_matrix, &mut rotated.p[i]);
            }

            // Calculate normal
            line1 = rotated.p[1] - rotated.p[0];
            line2 = rotated.p[2] - rotated.p[0];
            normal = line1.cross(&line2);
            normal.normalize();

            // For now, just check if z is positive
            if normal.z < 0.0 {

                // Translate
                translated.p[0] = rotated.p[0] + translation;
                translated.p[1] = rotated.p[1] + translation;
                translated.p[2] = rotated.p[2] + translation;
                // Project triangles from 3D to 2D
                for i in 0..3 {
                    translated.p[i].dot_by_matrix(&proj_matrix, &mut projected.p[i]);
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