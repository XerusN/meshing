use std::{fmt::Error, result};

use curves::bezier::NormalCurve;
use flo_canvas::*;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Coordinates {
    // Cartesian direct coordinate system
    pub x : f64,
    pub y : f64,
}

impl Coordinates {
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn normalize(&self) -> Coordinates{
        let norm = self.norm();
        Coordinates {
            x: self.x/norm,
            y: self.y/norm,
        }
    }
    
    pub fn dot_product(&self, other : &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    pub fn segment_to(&self, other : &Self) -> Coordinates {
        Coordinates{
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
    
    // With z pointing away from the drawing
    pub fn orthognal_segment(&self) -> Coordinates {
        Coordinates{
            x: self.y,
            y: - self.x,
        }
    }
    
    pub fn angle_with(&self, other: &Self) -> f64 {
        (self.dot_product(other) / (self.norm() * other.norm())).acos()
    }
    
}

pub struct Triangle {
    pub center : Option<Coordinates>,
    pub vertices : [Coordinates; 3],
    pub adjacencies : [Option<usize>; 3],
}

impl Copy for Triangle { }

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}

impl Triangle {
    
    pub fn compute_center(&mut self) {
        self.center = Some(Coordinates {
            x: (self.vertices[0].x + self.vertices[1].x + self.vertices[2].x) / 3.0,
            y: (self.vertices[0].y + self.vertices[1].y + self.vertices[2].y) / 3.0,
        });
    }
    
    pub fn normals(&self) -> [Coordinates; 3] {
        
        let normal1 = self.vertices[0].segment_to(&self.vertices[1]).orthognal_segment().normalize();
        let normal2 = self.vertices[1].segment_to(&self.vertices[2]).orthognal_segment().normalize();
        let normal3 = self.vertices[2].segment_to(&self.vertices[0]).orthognal_segment().normalize();
        
        [normal1, normal2, normal3]
    }
    
    pub fn edges(&self) -> [Coordinates; 3] {
        
        [
            self.vertices[0].segment_to(&self.vertices[1]),
            self.vertices[1].segment_to(&self.vertices[2]),
            self.vertices[2].segment_to(&self.vertices[0]),
        ]
        
    }
    
    pub fn vertices_to(&self, point : &Coordinates) -> [Coordinates; 3] {
        [
            self.vertices[0].segment_to(point),
            self.vertices[1].segment_to(point),
            self.vertices[2].segment_to(point)
        ]
    }
    
    pub fn signed_area(&self) -> f64 {
        0.5 * (-self.vertices[1].y * self.vertices[2].x + self.vertices[0].y * (-self.vertices[1].x + self.vertices[2].x) + self.vertices[0].x * (self.vertices[1].y - self.vertices[2].y) + self.vertices[1].x * self.vertices[2].y)
    }
    
    pub fn barycentric_coordinates_from(&self, point: &Coordinates) -> (f64, f64) {
        
        let s = ((self.vertices[1].y - self.vertices[2].y) * (point.x - self.vertices[2].x) + (self.vertices[2].x - self.vertices[1].x) * (point.y - self.vertices[2].y)) / ((self.vertices[1].y - self.vertices[2].y) * (self.vertices[0].x - self.vertices[2].x) + (self.vertices[2].x - self.vertices[1].x) * (self.vertices[0].y - self.vertices[2].y));
        let t = ((self.vertices[2].y - self.vertices[0].y) * (point.x - self.vertices[2].x) + (self.vertices[0].x - self.vertices[2].x) * (point.y - self.vertices[2].y)) / ((self.vertices[1].y - self.vertices[2].y) * (self.vertices[0].x - self.vertices[2].x) + (self.vertices[2].x - self.vertices[1].x) * (self.vertices[0].y - self.vertices[2].y));
        
        (s, t)
        
    }
    
    pub fn trilinear_coordinates_from(&self, point: &Coordinates) -> (f64, f64, f64) {
        
        let (s, t) = self.barycentric_coordinates_from(point);
        
        let a = s / self.vertices[0].segment_to(&self.vertices[1]).norm();
        let b = t / self.vertices[1].segment_to(&self.vertices[2]).norm();
        let c = (1.0 - s - t) / self.vertices[2].segment_to(&self.vertices[0]).norm();
        
        (a, b, c)
    }
    
    pub fn include(&self, point : &Coordinates) -> bool {
        
        let (s, t) = self.barycentric_coordinates_from(point);
        (s >= 0.0) & (t >= 0.0) & (1.0 - s - t >= 0.0)
    }
    
    pub fn draw(&self, window_dimension: &(Coordinates, Coordinates), canvas: &DrawingTarget, line_color: &Color) {
        
        canvas.draw(|gc| {
            // Set up the canvas
            gc.canvas_height((window_dimension.1.x - window_dimension.0.x) as f32);
            gc.center_region(window_dimension.0.x as f32, window_dimension.0.y as f32, window_dimension.1.x as f32, window_dimension.1.y as f32);

            // Draw a circle
            gc.new_path();
            gc.move_to(self.vertices[0].x as f32, self.vertices[0].y as f32);
            gc.line_to(self.vertices[1].x as f32, self.vertices[1].y as f32);
            gc.line_to(self.vertices[2].x as f32, self.vertices[2].y as f32);
            gc.line_to(self.vertices[0].x as f32, self.vertices[0].y as f32);
            
            // gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 0.0));
            // gc.fill();
            let width_factor: f32 = 0.001;
            gc.line_width(width_factor*(window_dimension.1.x - window_dimension.0.x) as f32);
            gc.stroke_color(*line_color);
            gc.stroke();
        });
        
    }
    
    pub fn find_face_to_point(&self, point: &Coordinates) -> Result<usize, &str>{
        
        let normals = self.normals();
        
        let mut closest_face: i64 = -1;
        let mut min_distance: f64 = 1000.0;  //might not work if the point field is not rescaled
        
        let distances = self.trilinear_coordinates_from(point);
        
        for i in 0..self.adjacencies.len() {
            
            match self.adjacencies[i] {
                None => (),
                _ => {
                    let distance = match i {
                        0 => distances.1.abs(),
                        1 => distances.2.abs(),
                        2 => distances.0.abs(),
                        _ => panic!(),
                    };
                    
                    if distance < min_distance {
                        min_distance = distance;
                        closest_face = i as i64;
                    }
                },
            }
        }
        
        let result;
        
        println!("distance : {:?}", min_distance);
        
        if closest_face < 0 {
            result = Err("No face to go to the point");
        } else {
            result = Ok(closest_face as usize);
        }
        
        result
            
    }
    
    pub fn print_triangle(&self) {
        println!("Current_triangle : [({:?}, {:?}), ({:?}, {:?}), ({:?}, {:?})]",
            self.vertices[0].x,
            self.vertices[0].y,
            self.vertices[1].x,
            self.vertices[1].y,
            self.vertices[2].x,
            self.vertices[2].y,
        );
    }
    
    pub fn circumcircle_radius(&self) -> f64{
        
        let mut radius = 0.0;
        for edge in self.edges() {
            radius *= edge.norm();
        }
        
        radius /= 4.0 * self.signed_area().abs();
        
        radius
        
    }
    
    pub fn is_point_in_circumucircle(&self, point: &Coordinates) -> bool {
        
        //dont know what's going on
        
        // let edges = self.edges();
        
        // let cos_a = - edges[2].dot_product(&edges[1]);
        // let cos_b = self.vertices[0].segment_to(point).dot_product(&self.vertices[1].segment_to(point));
        
        // let mut result;
        
        // if (cos_a >= 0.0) & (cos_b >= 0.0) {
        //     result = false;
        // } else if (cos_a < 0.0) & (cos_b < 0.0) {
        //     result = true;
        // } else {
        //     let sin_a = - edges[2].x * edges[1].y + edges[2].y * edges[1].x;
        //     let sin_b = self.vertices[0].segment_to(point).y * self.vertices[1].segment_to(point).x - self.vertices[0].segment_to(point).x * self.vertices[1].segment_to(point).y;
        //     let sin_ab = sin_a * cos_b + sin_b * cos_a;
        //     if sin_ab < 0.0 {
        //         result = true;
        //     } else {
        //         result = false;
        //     }
        // }
        
        // let pa = point.segment_to(&self.vertices[0]);
        // let pb = point.segment_to(&self.vertices[1]);
        // let pc = point.segment_to(&self.vertices[2]);
        
        // let angle_pab = pa.angle_with(&pb);
        // println!("{:?}", angle_pab);
        // let angle_pbc = pb.angle_with(&pc);
        // println!("{:?}", angle_pbc);
        // let angle_pca = pc.angle_with(&pa);
        // println!("{:?}", angle_pca);
        
        // let sum = angle_pab + angle_pbc + angle_pca;
        
        // println!("{:?}", sum);
        
        // let result;
        
        // if sum < 2.0 * PI {
        //     result = true;
        // } else {
        //     result = false;
        // }
        
        let pa = point.segment_to(&self.vertices[0]);
        let pb = point.segment_to(&self.vertices[1]);
        let pc = point.segment_to(&self.vertices[2]);
        
        //see the circumcircle wikipedia page, this is the determinant of a matrix which will tell if the point is inside or outside of the circumcircle
        let det = pa.dot_product(&pa) * (pb.x * pc.y - pc.x * pb.y)
            - pb.dot_product(&pb) * (pa.x * pc.y - pc.x * pa.y)
            + pc.dot_product(&pc) * (pa.x * pb.y - pb.x * pa.y)
        ;
        
        det > 0.0
        
    }
    
}

pub fn build_coordinates(x: f64, y: f64) -> Coordinates {
    Coordinates {
        x: x,
        y: y,
    }
}

pub fn build_triangle(center: Option<Coordinates>, vertices: [Coordinates; 3], adjacencies: [Option<usize>; 3]) -> Triangle {
    
    Triangle {
        center: center,
        vertices: vertices,
        adjacencies: adjacencies,
    }
}

