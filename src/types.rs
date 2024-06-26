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
    
    pub fn normalize(&self) -> Coordinates{
        let norm = self.norm();
        Coordinates {
            x: self.x/norm,
            y: self.y/norm,
        }
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
        
        let normal1 = self.vertices[0].segment_to(&self.vertices[1]).orthognal_segment().normalize();
        let normal2 = self.vertices[1].segment_to(&self.vertices[2]).orthognal_segment().normalize();
        let normal3 = self.vertices[2].segment_to(&self.vertices[0]).orthognal_segment().normalize();
        
        [normal1, normal2, normal3]
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
        
        let area = self.signed_area();
        
        println!("{:?}", area);
        
        let s = 1.0/(2.0*area)*(self.vertices[0].y * self.vertices[2].x - self.vertices[0].x * self.vertices[2].y + (self.vertices[2].y - self.vertices[0].y) * point.x + (self.vertices[0].x - self.vertices[2].x) * point.y);
        let t = 1.0/(2.0*area)*(self.vertices[0].x * self.vertices[1].y - self.vertices[0].x * self.vertices[1].y + (self.vertices[0].y - self.vertices[1].y) * point.x + (self.vertices[1].x - self.vertices[0].x) * point.y);
        
        (s, t)
        
    }
    
    pub fn include(&self, point : &Coordinates) -> bool {
        
        let (s, t) = self.barycentric_coordinates_from(point);
        println!("{:?} | {:?}", s, t);
        (s > 0.0) & (t > 0.0) & (1.0 - s - t > 0.0)
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