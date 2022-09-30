use std::time::{SystemTime, Duration};
// use std::vec;

// use rand::Rng;

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

const FPS_CAP: u64 = 60;


/// Application entry point
pub fn main() {
    // Create a new window
    let (mut canvas, mut event_pump) = init_sdl();

    let mut time_of_last_frame = SystemTime::now();

    // // RNG
    // let mut rng = rand::thread_rng();

    let z_near = 0.1; // Near clipping plane
    let z_far = 1000.0; // Far clipping plane
    let a = WIDTH as f32/HEIGHT as f32; // Aspect ratio
    let fov = 60.0; // Field of view
    let fov = 1.0 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();

    let model_mesh = Mesh::load_from_file("assets/teapot-trian.obj");
    let mut triangles_to_raster = Mesh::new(Vec::new());

    // Projection matrix
    let mut proj_matrix: Matrix4x4;

    // Rotation matrix
    let mut theta:f32 = 0.0;
    let mut rot_matrix_z: Matrix4x4;
    let mut rot_matrix_x: Matrix4x4;
    let mut rot_matrix: Matrix4x4;

    let mut light_dir = Vec3::from([-1.0, -1.0, -1.0]);
    light_dir.normalize();

    let translation = Vec3::from([0.0, 0.0, 8.0]);
    let camera = Vec3::from([0.0, 0.0, 0.0]);

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
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
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
        theta += 0.015;
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
        let mut rotated: Tri;
        let mut translated: Tri;
        let mut projected: Tri;
        let mut normal: Vec3;
        let mut vec_to_camera: Vec3;
        for triangle in model_mesh.tris.iter() {
            // Rotate
            rotated = *triangle * rot_matrix;

            // Translate
            translated = rotated + translation;

            // Calculate normal
            normal = translated.normal();
            // Calculate vector from camera to triangle
            vec_to_camera = translated.p[0] - camera;
            if normal.dot(&vec_to_camera) < 0.0 {
                // Light
                // Calculate light intensity
                let light_intensity = normal.dot(&light_dir) + 1.0;

                // Project triangles from 3D to 2D
                projected = translated * proj_matrix;

                // Scale into view
                for i in 0..3 {
                    projected.p[i].x += 1.0;
                    projected.p[i].y += 1.0;
                    projected.p[i].x *= 0.5 * WIDTH as f32;
                    projected.p[i].y *= 0.5 * HEIGHT as f32;
                }

                projected.c = Option::Some(Color::RGB(
                    (light_intensity * 127.0) as u8,
                    (light_intensity * 127.0) as u8,
                    (light_intensity * 127.0) as u8
                ));

                // Store triangle for rastering later
                triangles_to_raster.tris.push(projected);
            }
        }

        // Sort triangles from back to front
        triangles_to_raster.sort();
        // Draw triangles to screen
        for triangle in triangles_to_raster.tris.iter() {
            triangle.draw_filled(&mut canvas);
            // triangle.draw(&mut canvas, triangle.c.unwrap());
        }

        // Clear triangles
        triangles_to_raster.tris.clear();

        canvas.present();

        // Cap FPS
        let time_since_last_frame = SystemTime::now().duration_since(time_of_last_frame).unwrap();
        if time_since_last_frame < Duration::from_millis(1000 / FPS_CAP) {
            std::thread::sleep(Duration::from_millis(1000 / FPS_CAP) - time_since_last_frame);
        }
        time_of_last_frame = SystemTime::now();
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