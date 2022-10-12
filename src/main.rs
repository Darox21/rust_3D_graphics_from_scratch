use std::time::{SystemTime, Duration};

extern crate nalgebra as na;
// use na::Transform3;
use na::{Vector3, Matrix4};//, U3, U4, DefaultAllocator, allocator::Allocator};

mod utilities;
use utilities::{Mesh, matrices};

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
const BG_COLOR: Color = Color::RGB(15, 17, 17);
const TITLE: &str = "Rust 3D Renderer";

/// Application entry point
pub fn main() {
    // Create a new window
    let (mut canvas, mut event_pump) = init_sdl();
    let mut time_of_last_frame = SystemTime::now();

    // Load the mesh
    let model_mesh = Mesh::load_from_file("assets/teapot-trian.obj");

    // Matrices
    let aspect_ratio = WIDTH as f32/HEIGHT as f32; // Aspect ratio
    let proj_matrix = matrices::projection_matrix(60.0, aspect_ratio, 0.1, 1000.0);
    let translation_matrix = matrices::translation_matrix(0.0, 0.0, 6.0);
    let mut world_matrix: Matrix4<f32>;
    let camera = Vector3::new(0.0, 0.0, 0.0);
    let mut theta:f32 = 0.0; // Rotations

    // Lighting
    let mut light_dir = Vector3::new(-1.0, -1.0, 0.0);
    light_dir.normalize_mut();
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

        // Clear the screen
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();

        // Update Matrices
        theta += 0.015;
        let rot_matrix_z = matrices::rotation_matrix_z(theta);
        let rot_matrix_x = matrices::rotation_matrix_x(theta);

        // Calculate world matrix
        world_matrix = rot_matrix_z * rot_matrix_x;
        world_matrix = translation_matrix * world_matrix;

        // Draw Triangles
        let mut triangles_to_raster = Mesh::new(Vec::new());
        for triangle in model_mesh.tris.iter() {
            // transform triangle
            let mut transformed = triangle.clone();
            transformed *= world_matrix;

            // Translate triangle
            // transformed += translation;

            // Calculate normal
            let normal = transformed.normal();
            // Calculate vector from camera to triangle
            let mut vec_to_camera = camera - transformed.p[0].xyz();
            vec_to_camera.normalize_mut();
            if normal.dot(&vec_to_camera) > 0.0 {
                // Light and Color
                let light_intensity = normal.dot(&light_dir) + 1.0;
                transformed.c = Option::Some(Color::RGB(
                    (light_intensity * 127.0) as u8,
                    (light_intensity * 127.0) as u8,
                    (light_intensity * 127.0) as u8
                ));

                // Project triangles from 3D to 2D
                let mut projected = proj_matrix.transpose() * transformed;
                // Normalize the projected triangle
                projected.p[0] /= projected.p[0].w;
                projected.p[1] /= projected.p[1].w;
                projected.p[2] /= projected.p[2].w;

                // Scale into view
                for i in 0..3 {
                    projected.p[i].x += 1.0;
                    projected.p[i].y += 1.0;
                    projected.p[i].x *= 0.5 * WIDTH as f32;
                    projected.p[i].y *= 0.5 * HEIGHT as f32;
                }

                // Store triangle for rastering later
                triangles_to_raster.tris.push(projected);
            }
        }

        // Draw triangles to screen
        triangles_to_raster.sort();
        draw_mesh(&triangles_to_raster, &mut canvas);
        triangles_to_raster.tris.clear();

        // Update the screen
        canvas.present();

        // Cap FPS
        limit_fps(&mut time_of_last_frame, FPS_CAP);
    }
}

fn draw_mesh(mesh: &Mesh, canvas: &mut Canvas<Window>) {
    for triangle in mesh.tris.iter() {
        // // Draw half the screen with draw, the other half with draw_gfx
        // if triangle.midpoint().x < WIDTH as f32/2.0 {
        //     triangle.draw_gfx(canvas);
        // } else {
            triangle.draw(canvas);
        // }
    }
}

fn limit_fps(prev_time: &mut SystemTime, fps_cap: u64) {
    let time_difference = SystemTime::now().duration_since(*prev_time).unwrap();
    if time_difference < Duration::from_millis(1000 / fps_cap) {
        std::thread::sleep(Duration::from_millis(1000 / fps_cap) - time_difference);
    }
    *prev_time = SystemTime::now();
}

/// Initialize SDL
fn init_sdl() -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(TITLE, WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump)
}
