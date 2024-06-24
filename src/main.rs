struct Coordinates {
    // Cartesian direct coordinate system
    x : f64,
    y : f64,
}

impl Coordinates {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
    }
    
    fn dot_product(&self, other : &Coordinates) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    fn segment_to(&self, other : &Coordinates) -> Coordinates {
        Coordinates{
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
    
    // With z pointing away from the drawing
    fn orthognal_segment(&self) -> Coordinates {
        Coordinates{
            x: self.y,
            y: - self.x,
        }
    }
}

struct Triangle {
    center : Option<Coordinates>,
    vertices : [Coordinates; 3],
    adjacencies : [Option<u32>; 3],
}

impl Triangle {
    fn normals(&self) -> [Coordinates; 3] {
        
        let normal1 = self.vertices[0].segment_to(&self.vertices[1]).orthognal_segment();
        let normal2 = self.vertices[1].segment_to(&self.vertices[2]).orthognal_segment();
        let normal3 = self.vertices[2].segment_to(&self.vertices[0]).orthognal_segment();
        
        [normal1, normal2, normal3]
    }
    
    fn vertices_to(&self, point : &Coordinates) -> [Coordinates; 3] {
        [
            self.vertices[0].segment_to(point),
            self.vertices[1].segment_to(point),
            self.vertices[2].segment_to(point)
        ]
    }
    
    fn include(&self, point : &Coordinates) -> bool {
        let normals = self.normals();
        let vertices_to_point = self.vertices_to(point);
        let mut check = 0;
        for i in 0..normals.len() {
            if (normals[i].dot_product(&vertices_to_point[i]) >= 0.0) {
                check += 1;
            }
        }
        
        check == 3
    }
}




fn main() {
    let triangle = Triangle{
        center : None,
        vertices : [a, b, c],
        adjacencies : [None, None, None],
    };
}
