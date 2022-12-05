/*!
# polygons.rs

A library of utilities for the renderer.
(should compartimentalize this)

Implementations for the `Tri` and `Mesh` structs, such as
`Mesh::load_from_file()` and `Tri::draw()`.
*/
use na::{Vector3, Vector4, Matrix4, Matrix, U4, ArrayStorage};


use std::ops::{AddAssign, Mul, MulAssign, DivAssign};
use std::fs;
use std::io::Read;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Point;


use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
// use sdl2::rect::Point;


/// A 3D triangle, with 3 vertices
/// and an optional color
#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub p: [Vector4<f32>; 3],
    pub c: Option<Color>
}

/// A Vector of 3D triangles
pub struct Mesh {
    pub tris: Vec<Tri>
}

impl Tri {
    /// Creates a new triangle with the given vertices
    #[allow(dead_code)]
    pub fn new(p1: Vector4<f32>, p2: Vector4<f32> , p3: Vector4<f32>, color: Color) -> Self {
        Self {
            p: [p1, p2, p3],
            c: Some(color)
        }
    }

    /// Calculates the normal of the triangle
    pub fn normal(&self) -> Vector3<f32> {
        let line1 = self.p[1] - self.p[0];
        let line2 = self.p[2] - self.p[0];
        // Because the cross product is only defined for 3D vectors,
        // we need to ignore the 4th dimension
        let line1 = Vector3::new(line1.x, line1.y, line1.z);
        let line2 = Vector3::new(line2.x, line2.y, line2.z);
        line1.cross(&line2).normalize()
    }

    /// Calculates the center of the triangle
    pub fn midpoint(&self) -> Vector3<f32> {
        let mut midpoint = Vector3::new(0.0, 0.0, 0.0);
        for i in 0..3 {
            midpoint.x += self.p[i].x;
            midpoint.y += self.p[i].y;
            midpoint.z += self.p[i].z;
        }
        midpoint /= 3.0;
        midpoint
    }

    /// Draws the filled triangle to the given canvas with a local
    /// implementation of the bresenham line algorithm
    #[allow(dead_code)]
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.c.unwrap_or(Color::GREEN));
        // Due to visual artifacts, we need to draw the triangle from scratch
        // So we can't use the `gfx::primitives::filled_triangle` function

        // Use the Bresenham algorithm to get the outline of the triangle
        let mut outline = Vec::new();
        for i in 0..3 {
            let p1 = self.p[i];
            let p2 = self.p[(i + 1) % 3];
            let mut line = Self::bresenham_line(p1.x as i16, p1.y as i16, p2.x as i16, p2.y as i16);
            outline.append(&mut line);
        }

        // Now we can use the scanline algorithm to fill the triangle
        Self::scanline_fill(outline, canvas);
    }

    /// Draws only the outline of the triangle to the given canvas
    pub fn draw_outline(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.c.unwrap_or(Color::GREEN));

        // Use the Bresenham algorithm to get the outline of the triangle
        let mut outline = Vec::new();
        for i in 0..3 {
            let p1 = self.p[i];
            let p2 = self.p[(i + 1) % 3];
            let mut line = Self::bresenham_line(p1.x as i16, p1.y as i16, p2.x as i16, p2.y as i16);
            outline.append(&mut line);
        }

        for point in outline.iter() {
            canvas.draw_point(Point::new(point[0] as i32, point[1] as i32)).unwrap();
        }
    }

    /// Uses the sdl2::gfx::primitives::filled_trigon function to draw the triangle
    pub fn draw_gfx(&self, canvas: &mut Canvas<Window>) {
        let color = self.c.unwrap_or(Color::GREEN);
        canvas.set_draw_color(color);
        canvas.filled_trigon(
            self.p[0].x as i16, self.p[0].y as i16,
            self.p[1].x as i16, self.p[1].y as i16,
            self.p[2].x as i16, self.p[2].y as i16,
            color
        ).unwrap();
    }

    /// Uses the Bresenham algorithm to output a list of pixels
    /// that make up the line between the two given points
    ///
    /// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn bresenham_line(x1: i16, y1: i16, x2: i16, y2: i16) -> Vec<[i16; 2]> {
        let mut line = Vec::new();
        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            line.push([x, y]);
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        line
    }

    /// Uses the scanline algorithm to fill the given outline
    /// with the given triangle color
    fn scanline_fill(mut outline: Vec<[i16; 2]>, canvas: &mut Canvas<Window>) {
        // Sort the outline by y coordinate
        outline.sort_by(|a, b| a[1].cmp(&b[1]));

        // Current y coordinate
        let mut y = outline[0][1];
        // Current x coordinates of the left and right edges
        let mut min_x = outline[0][0];
        let mut max_x = outline[0][0];
        for point in outline.iter() {
            // If we have not reached the next scanline
            if point[1] == y {
                // Update the x coordinates
                if point[0] < min_x {
                    min_x = point[0];
                }
                if point[0] > max_x {
                    max_x = point[0];
                }
            } else {
                // We have reached the next scanline
                // Draw the line between the left and right edges
                canvas.hline(min_x, max_x, y, canvas.draw_color()).unwrap();
                // Update the y coordinate
                y = point[1];
                // Update the x coordinates
                min_x = point[0];
                max_x = point[0];
            }
        }
    }
}

impl From<[f32; 9]> for Tri {
    fn from(p: [f32; 9]) -> Self {
        Self {
            p: [
                Vector4::new(p[0], p[1], p[2], 1.0),
                Vector4::new(p[3], p[4], p[5], 1.0),
                Vector4::new(p[6], p[7], p[8], 1.0)
            ],
            c: None
        }
    }
}

impl From<[Vector4<f32>; 3]> for Tri {
    fn from(p: [Vector4<f32>; 3]) -> Self {
        Self {
            p: [p[0], p[1], p[2]],
            c: None
        }
    }
}

impl AddAssign<Vector4<f32>> for Tri {
    fn add_assign(&mut self, rhs: Vector4<f32>) {
        for i in 0..3 {
            self.p[i] += rhs;
        }
    }
}

impl Mul<Matrix<f32, U4, U4, ArrayStorage<f32, 4, 4>>> for Tri {
    type Output = Self;

    fn mul(self, other: Matrix4<f32>) -> Self {
        Self {
            p: [
                other * self.p[0],
                other * self.p[1],
                other * self.p[2]
            ],
            c: self.c
        }
    }
}

impl Mul<Tri> for Matrix4<f32> {
    type Output = Tri;

    fn mul(self, other: Tri) -> Tri {
        // Multiply each point by the matrix
        Tri {
            p: [
                self * other.p[0],
                self * other.p[1],
                self * other.p[2]
            ],
            c: other.c
        }
    }
}

impl MulAssign<Matrix4<f32>> for Tri {
    fn mul_assign(&mut self, rhs: Matrix4<f32>) {
        // Multiply each point by the matrix
        self.p[0] = rhs * self.p[0];
        self.p[1] = rhs * self.p[1];
        self.p[2] = rhs * self.p[2];
    }
}

impl DivAssign<f32> for Tri {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..3 {
            self.p[i] /= rhs;
        }
    }
}


impl Mesh {
    /// Creates a new mesh from a list of triangles
    pub fn new(tris: Vec<Tri>) -> Self {
        Self { tris }
    }

    /// Loads a mesh from a .obj file
    pub fn load_from_file(filename: &str) -> Self {
        let mut tris = Vec::new();
        let mut tri: [Vector4<f32>; 3];
        let mut vertex: Vector4<f32>;
        let mut buffer: Vec<Vector4<f32>> = Vec::new();
        let mut relations: [Vec<i32>;3];
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
                    vertex = Vector4::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap(), 1.0);
                    buffer.push(vertex);
                },
                ["f", v1, v2, v3, ..] => {
                    // A triangle
                    relations = [
                        v1.split("/").map(|s| s.parse().unwrap()).collect(),
                        v2.split("/").map(|s| s.parse().unwrap()).collect(),
                        v3.split("/").map(|s| s.parse().unwrap()).collect()
                    ];
                    tri = [
                        buffer[relations[0][0] as usize - 1],
                        buffer[relations[1][0] as usize - 1],
                        buffer[relations[2][0] as usize - 1]
                    ];
                    tris.push(Tri::from(tri));
                },
                _ => ()
            }
        }
        println!("Loaded {} triangles", tris.len());
        Self::new(tris)
    }

    /// Sorts the triangles in the mesh by their average z coordinate
    pub fn sort(&mut self) {
        self.tris.sort_by(|a, b| {
            let dist_a = (a.p[0].z + a.p[1].z + a.p[2].z) / 3.0;
            let dist_b = (b.p[0].z + b.p[1].z + b.p[2].z) / 3.0;
            dist_b.partial_cmp(&dist_a).unwrap()
        });
    }

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tri() {
        let t1 = Tri::from([ 1.0,-1.0, 1.0, 1.0, 1.0, 1.0,-1.0, 1.0, 1.0]);
        let t2 = Tri::from([-1.0,-1.0, 1.0,-1.0, 1.0, 1.0,-1.0, 1.0,-1.0]);

        assert_eq!(t1.normal(), Vector4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(t2.normal(), Vector4::new(-1.0, 0.0, 0.0, 0.0));
    }

    // Draw a triangle
    // #[test]
    // fn test_draw() {
    //     let mut t = Tri::from([ 1.0,-1.0, 1.0, 1.0, 1.0, 1.0,-1.0, 1.0, 1.0]);
    //     let mut m = matrices::translation_matrix(0.0, 0.0, 3.0);
    //     let mut screen = Screen::new(100, 100);
    //     screen.draw(&t);
    //     screen.print();
    // }
}