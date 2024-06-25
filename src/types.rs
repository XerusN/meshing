use curves::line;
use flo_canvas::*;

pub struct Coordinates {
    // Cartesian direct coordinate system
    pub x : f64,
    pub y : f64,
}

impl Coordinates {
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
    }
    
    pub fn dot_product(&self, other : &Coordinates) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    pub fn segment_to(&self, other : &Coordinates) -> Coordinates {
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
    
}

pub struct Triangle {
    pub center : Option<Coordinates>,
    pub vertices : [Coordinates; 3],
    pub adjacencies : [Option<u32>; 3],
}

impl Triangle {
    pub fn normals(&self) -> [Coordinates; 3] {
        
        let normal1 = self.vertices[0].segment_to(&self.vertices[1]).orthognal_segment();
        let normal2 = self.vertices[1].segment_to(&self.vertices[2]).orthognal_segment();
        let normal3 = self.vertices[2].segment_to(&self.vertices[0]).orthognal_segment();
        
        [normal1, normal2, normal3]
    }
    
    pub fn vertices_to(&self, point : &Coordinates) -> [Coordinates; 3] {
        [
            self.vertices[0].segment_to(point),
            self.vertices[1].segment_to(point),
            self.vertices[2].segment_to(point)
        ]
    }
    
    pub fn include(&self, point : &Coordinates) -> bool {
        let normals = self.normals();
        let vertices_to_point = self.vertices_to(point);
        let mut check = 0;
        for i in 0..normals.len() {
            if normals[i].dot_product(&vertices_to_point[i]) >= 0.0 {
                check += 1;
            }
        }
        
        check == 3
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
}

pub fn build_coordinates(x: f64, y: f64) -> Coordinates {
    Coordinates {
        x: x,
        y: y,
    }
}

pub fn build_triangle(center: Option<Coordinates>, vertices: [Coordinates; 3], adjacencies: [Option<u32>; 3]) -> Triangle {
    Triangle {
        center: center,
        vertices: vertices,
        adjacencies: adjacencies,
    }
    
}