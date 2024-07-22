//used to draw the triangles
use flo_canvas::*;

use crate::types::cells::cell_trait::*;

use crate::types::base::*;

pub struct Triangle {
    pub center : Option<Point>,
    pub vertices : [Point; 3],
    pub adjacencies : [Option<Neighbor>; 3],
}

impl Copy for Triangle { }

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}

impl Cell for Triangle {
    fn include(&self, point : &Point) -> bool {
        let (s, t) = self.barycentric_coordinates_from(point);
        (s >= 0.0) & (t >= 0.0) & (1.0 - s - t >= 0.0)
    }
}

impl Triangle {
    
    pub fn compute_center(&mut self) {
        self.center = Some(&(&(&self.vertices[0] + &self.vertices[1]) + &self.vertices[2]) / 3.0)
    }
    
    pub fn normals(&self) -> [Vector; 3] {
        
        let normal1 = self.vertices[0].segment_to(&self.vertices[1]).orthognal_segment().normalize();
        let normal2 = self.vertices[1].segment_to(&self.vertices[2]).orthognal_segment().normalize();
        let normal3 = self.vertices[2].segment_to(&self.vertices[0]).orthognal_segment().normalize();
        
        [normal1, normal2, normal3]
    }
    
    pub fn edges(&self) -> [Vector; 3] {
        
        [
            self.vertices[0].segment_to(&self.vertices[1]),
            self.vertices[1].segment_to(&self.vertices[2]),
            self.vertices[2].segment_to(&self.vertices[0]),
        ]
        
    }
    
    pub fn vertices_to(&self, point : &Point) -> [Vector; 3] {
        [
            self.vertices[0].segment_to(point),
            self.vertices[1].segment_to(point),
            self.vertices[2].segment_to(point)
        ]
    }
    
    pub fn signed_area(&self) -> f64 {
        0.5 * (-self.vertices[1].y * self.vertices[2].x + self.vertices[0].y * (-self.vertices[1].x + self.vertices[2].x) + self.vertices[0].x * (self.vertices[1].y - self.vertices[2].y) + self.vertices[1].x * self.vertices[2].y)
    }
    
    pub fn barycentric_coordinates_from(&self, point: &Point) -> (f64, f64) {
        
        let s = ((self.vertices[1].y - self.vertices[2].y) * (point.x - self.vertices[2].x) + (self.vertices[2].x - self.vertices[1].x) * (point.y - self.vertices[2].y)) / ((self.vertices[1].y - self.vertices[2].y) * (self.vertices[0].x - self.vertices[2].x) + (self.vertices[2].x - self.vertices[1].x) * (self.vertices[0].y - self.vertices[2].y));
        let t = ((self.vertices[2].y - self.vertices[0].y) * (point.x - self.vertices[2].x) + (self.vertices[0].x - self.vertices[2].x) * (point.y - self.vertices[2].y)) / ((self.vertices[1].y - self.vertices[2].y) * (self.vertices[0].x - self.vertices[2].x) + (self.vertices[2].x - self.vertices[1].x) * (self.vertices[0].y - self.vertices[2].y));
        
        (s, t)
        
    }
    
    pub fn trilinear_coordinates_from(&self, point: &Point) -> (f64, f64, f64) {
        
        let (s, t) = self.barycentric_coordinates_from(point);
        
        //false according to wikipedia
        // let a = s / self.vertices[0].segment_to(&self.vertices[1]).norm();
        // let b = t / self.vertices[1].segment_to(&self.vertices[2]).norm();
        // let c = (1.0 - s - t) / self.vertices[2].segment_to(&self.vertices[0]).norm();
        
        let a = s / self.vertices[1].segment_to(&self.vertices[2]).norm();
        let b = t / self.vertices[2].segment_to(&self.vertices[0]).norm();
        let c = (1.0 - s - t) / self.vertices[0].segment_to(&self.vertices[1]).norm();
        
        (a, b, c)
    }
    
    pub fn draw(&self, window_dimension: &(Point, Point), canvas: &DrawingTarget, line_color: &Color) {
        
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
    
    pub fn find_face_to_point(&self, point: &Point, old_triangle: i64) -> Result<usize, &str>{
        
        // Not working yet, wip. But should be broken no matter what when implementing the constrained version of the triangulation
        
        let normals = self.normals();
        
        let mut closest_face: i64 = -1;
        let mut min_distance: f64 = 1000.0;  //might not work if the point field is not rescaled
        
        let distances = self.trilinear_coordinates_from(point);
        
        self.print_triangle();
        println!("Distances : {:?}", distances);
        
        for i in 0..self.adjacencies.len() {
            if i as i64 == old_triangle {
                //continue;
            }
            
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
        
        // let mut closest_face: i64 = -1;
        // let mut min_distance: f64 = 1000.0;
        
        // for i in 0..self.adjacencies.len() {
            
        //     match self.adjacencies[i] {
        //         None => (),
        //         _ => {
        //             let line = self.vertices[i].segment_to(&self.vertices[(i + 1) % 3]);
                    
        //             let distance = (line.y * point.x - line.x * point.y + line.x * self.vertices[i].y - line.y * self.vertices[i].x) / line.norm();
                    
        //             if distance.abs() < min_distance {
        //                 min_distance = distance.abs();
        //                 closest_face = i as i64;
        //             }
        //         },
        //     }
        // }
        
        // let mut closest_face: i64 = -1;
        // let mut min_distance: f64 = 1000.0;
        
        // let (s, t) = self.barycentric_coordinates_from(point);
        // let area = self.signed_area();
        
        // for i in 0..self.adjacencies.len() {
            
        //     match self.adjacencies[i] {
        //         None => (),
        //         _ => {
        //             let line = self.vertices[i].segment_to(&self.vertices[(i + 1) % 3]);
                    
        //             let distance = ((1.0 - s - t).abs() * 2.0 * area) / self.vertices[i].dot_product(&self.vertices[(i + 1) % 3]);
                    
        //             if distance.abs() < min_distance {
        //                 min_distance = distance.abs();
        //                 closest_face = i as i64;
        //             }
        //         },
        //     }
        // }
        
        let result;
        
        //println!("distance : {:?}", min_distance);
        
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
        println!("Adjacencies : {:?}", self.adjacencies);
    }
    
    pub fn circumcircle_radius(&self) -> f64{
        
        let mut radius = 0.0;
        for edge in self.edges() {
            radius *= edge.norm();
        }
        
        radius /= 4.0 * self.signed_area().abs();
        
        radius
    }
    
    pub fn is_point_in_circumucircle(&self, point: &Point) -> bool {
        
        let pa = point.segment_to(&self.vertices[0]);
        let pb = point.segment_to(&self.vertices[1]);
        let pc = point.segment_to(&self.vertices[2]);
        
        //see the circumcircle wikipedia page, this is the determinant of a matrix which will tell if the point is inside or outside of the circumcircle
        // or https://stackoverflow.com/questions/39984709/how-can-i-check-wether-a-point-is-inside-the-circumcircle-of-3-points
        let det = &pa * &pa * (pb.x * pc.y - pc.x * pb.y)
            - &pb * &pb * (pa.x * pc.y - pc.x * pa.y)
            + &pc * &pc * (pa.x * pb.y - pb.x * pa.y)
        ;
        
        det > 0.0
    }
    
    pub fn find_point_in_triangle(&self, point: &Point) -> Option<usize> {
        
        for i in 0..self.vertices.len() {
            if point == &self.vertices[i] {
                return Some(i);
            }
        }
        
        None
    }
    
    pub fn find_face_opposite_to(&self, point_local_id: usize) -> Option<Neighbor> {
        
        self.adjacencies[(point_local_id + 1) % 3]
        
    }
    
    pub fn find_point_local_id_opposite_to(&self, adjacent_triangle_id: usize) -> Option<usize> {
        for i in 0..3 {
            if self.adjacencies[i] == Some(Neighbor::Cell(adjacent_triangle_id)) {
                return Some((i + 2) % 3);
            }
        }
        
        None
    }
    
}

pub fn build_triangle(center: Option<Point>, vertices: [Point; 3], adjacencies: [Option<Neighbor>; 3]) -> Triangle {
    
    Triangle {
        center: center,
        vertices: vertices,
        adjacencies: adjacencies,
    }
}