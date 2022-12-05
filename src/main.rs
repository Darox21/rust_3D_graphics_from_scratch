/*!
# 3D Graphics from Scratch

* Version: 0.1.2
* Author: Darío Quiñones

A basic cpu renderer written in Rust.
This is a port loosely based in the C tutorial series by Javidx9
https://www.youtube.com/watch?v=ih20l3pJoeU

This is a work in progress, used mostly for me to understand the
aplications of linear algebra in computer graphics.
*/

use std::time::{SystemTime, Duration};

extern crate nalgebra as na;
use na::{Vector3};//, U3, U4, DefaultAllocator, allocator::Allocator};

mod polygons;
use polygons::Mesh;
mod linear_transforms;

use sdl2;
use sdl2::{
    pixels::Color,
    event::Event,
    video::Window,
    render::Canvas,
    EventPump,
    keyboard::Keycode,
    rect::Rect,
};


// Window size
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FPS_CAP: u64 = 60;
const BG_COLOR: Color = Color::RGB(15, 17, 17);
const TITLE: &str = "Rust 3D Renderer";

const VELOCITY_CAP: f32 = 0.15;

/// Application entry point
pub fn main() {
    // Create a new window
    let (mut canvas, mut event_pump) = init_sdl();
    let mut time_of_last_frame = SystemTime::now();

    // Load the mesh
    let model_mesh = Mesh::load_from_file("assets/teapot-trian.obj");

    // Matrices
    let aspect_ratio = WIDTH as f32/HEIGHT as f32; // Aspect ratio
    let proj_matrix = linear_transforms::projection_matrix(60.0, aspect_ratio, 0.1, 1000.0);
    let translation_matrix = linear_transforms::translation_matrix(0.0, 0.0, -8.0);

    let mut camera = Vector3::new(0.0, 0.0, 0.0);

    let mut theta:f32 = 0.0; // Rotations

    let mut moving: [bool; 6] = [false; 6]; // Up, Down, W, A, S, D
    let mut velocity = Vector3::new(0.0, 0.0, 0.0);

    // Lighting
    let mut light_dir = Vector3::new(-1.0, -1.0, 0.0);
    light_dir.normalize_mut();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Up and down
                Event::KeyDown { keycode: Some(Keycode::Space), ..} =>  {moving[0] = true;},
                Event::KeyUp { keycode: Some(Keycode::Space), ..} =>    {moving[0] = false;},
                Event::KeyDown { keycode: Some(Keycode::LShift), ..} => {moving[1] = true;},
                Event::KeyUp { keycode: Some(Keycode::LShift), ..} =>   {moving[1] = false;},
                // Forward and backward
                Event::KeyDown { keycode: Some(Keycode::W), ..} =>      {moving[2] = true;},
                Event::KeyUp { keycode: Some(Keycode::W), ..} =>        {moving[2] = false;},
                Event::KeyDown { keycode: Some(Keycode::S), ..} =>      {moving[3] = true;},
                Event::KeyUp { keycode: Some(Keycode::S), ..} =>        {moving[3] = false;},
                // Left and right
                Event::KeyDown { keycode: Some(Keycode::A), ..} =>      {moving[4] = true;},
                Event::KeyUp { keycode: Some(Keycode::A), ..} =>        {moving[4] = false;},
                Event::KeyDown { keycode: Some(Keycode::D), ..} =>      {moving[5] = true;},
                Event::KeyUp { keycode: Some(Keycode::D), ..} =>        {moving[5] = false;},
                _ => {}
            }
        }
        // Movement
        add_movement(&moving, &mut velocity);
        camera += velocity;

        // Clear the screen
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();

        // Update Matrices
        theta += 0.015;
        let rot_matrix_z = linear_transforms::rotation_matrix_z(theta * 0.7);
        let rot_matrix_x = linear_transforms::rotation_matrix_x(theta);

        // Calculate world matrix
        let mut world_matrix = rot_matrix_z * rot_matrix_x;
        world_matrix = translation_matrix * world_matrix;

        // // View matrix
        let looking_at = Vector3::new(0.0, 0.0, -1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        // // println!("Camera: {:?}", camera);
        // // let target = camera + looking_at;
        let view_matrix = linear_transforms::view_matrix(camera, looking_at, up);
        // println!(
        //     "View matrix: x: {:?}, y: {:?}, z: {:?}",
        //     view_matrix.m41,
        //     view_matrix.m42,
        //     view_matrix.m43
        // );

        // Draw Triangles
        let mut triangles_to_raster = Mesh::new(Vec::new());
        for triangle in model_mesh.tris.iter() {
            // transform triangle
            let mut transformed = triangle.clone();
            transformed *= world_matrix;

            // Move triangle to camera
            let mut viewed = view_matrix * transformed;

            // Calculate normal
            let normal = viewed.normal();
            // Calculate vector from camera to triangle
            let mut vec_to_camera = viewed.p[0].xyz() - camera;
            vec_to_camera.normalize_mut();
            if normal.dot(&vec_to_camera) < 0.0 {
                // Light and Color
                let light_intensity = normal.dot(&light_dir) + 1.0;
                viewed.c = Option::Some(Color::RGB(
                    (light_intensity * 127.0) as u8,
                    (light_intensity * 127.0) as u8,
                    (light_intensity * 127.0) as u8
                ));

                // Project triangles from 3D to 2D
                let mut projected = proj_matrix * viewed;
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

        // Draw a Rect in the position of the camera
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(Rect::new(
            ((camera.x + 1.0) * 0.5 * WIDTH as f32) as i32,
            ((camera.y + 1.0) * 0.5 * HEIGHT as f32) as i32,
            4, 4
        )).unwrap();

        // Update the screen
        canvas.present();

        // Cap FPS
        limit_fps(&mut time_of_last_frame, FPS_CAP);
    }
}

/// Add movement to velocity
fn add_movement(moving: &[bool; 6], velocity: &mut Vector3<f32>) {
    if velocity.magnitude() < VELOCITY_CAP {
        let mut acceleration = Vector3::new(0.0, 0.0, 0.0);
        if moving[0] {acceleration += Vector3::new(0.0, -1.0, 0.0);}
        if moving[1] {acceleration += Vector3::new(0.0, 1.0, 0.0);}
        if moving[2] {acceleration += Vector3::new(0.0, 0.0, 1.0);}
        if moving[3] {acceleration += Vector3::new(0.0, 0.0, -1.0);}
        if moving[4] {acceleration += Vector3::new(-1.0, 0.0, 0.0);}
        if moving[5] {acceleration += Vector3::new(1.0, 0.0, 0.0);}

        if acceleration.magnitude() > 0.0 {
            acceleration *= 0.02;
            *velocity += acceleration;
        }
    }
    // Some friction
    *velocity *= 0.8;
}

/// Draw a mesh to the canvas given
fn draw_mesh(mesh: &Mesh, canvas: &mut Canvas<Window>) {
    for triangle in mesh.tris.iter() {
        // Draw half the screen with draw, the other half with draw_gfx
        // if triangle.midpoint().x > WIDTH as f32/1.7 {
        //     triangle.draw_outline(canvas);
        // } else {
        triangle.draw(canvas);
        // }
    }
}

/// Limit the FPS to the given cap
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
