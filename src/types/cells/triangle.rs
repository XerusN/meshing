//used to draw the triangles (test purpose)
use flo_canvas::*;

use crate::types::cells::cell_trait::*;

use crate::types::base::*;

/// Definition of the cell
pub struct Triangle {
    pub vertices: [Point; 3],
    pub adjacencies: [Option<Neighbor>; 3],
}

impl Copy for Triangle {}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}

impl Cell for Triangle {
    /// Checks if the point is in this cell
    fn include(&self, point: &Point) -> bool {
        let (s, t) = self.barycentric_coordinates_from(point);
        (s >= 0.0) & (t >= 0.0) & (1.0 - s - t >= 0.0)
    }

    /// Gives an iterator on all vertices of the cell
    fn center(&self) -> Point {
        &(&(&self.vertices[0] + &self.vertices[1]) + &self.vertices[2]) / 3.0
    }

    /// Gives an iterator on all neighbors of the cell
    fn iter_vertices(&self) -> std::slice::Iter<Point> {
        self.vertices.iter()
    }

    /// Gives the centroid of the cell
    fn iter_adjacencies(&self) -> std::slice::Iter<Option<Neighbor>> {
        self.adjacencies.iter()
    }

    /// Gives the normals to each edge of the cell
    fn normals(&self) -> Vec<Vector> {
        let normal1 = self.vertices[0]
            .segment_to(&self.vertices[1])
            .orthogonal_vector()
            .normalize();
        let normal2 = self.vertices[1]
            .segment_to(&self.vertices[2])
            .orthogonal_vector()
            .normalize();
        let normal3 = self.vertices[2]
            .segment_to(&self.vertices[0])
            .orthogonal_vector()
            .normalize();

        vec![normal1, normal2, normal3]
    }

    /// Gives the siged area of the cell.
    /// Is positive if the vertices are defined counter-clockwise.
    fn signed_area(&self) -> f64 {
        0.5 * (-self.vertices[1].y * self.vertices[2].x
            + self.vertices[0].y * (-self.vertices[1].x + self.vertices[2].x)
            + self.vertices[0].x * (self.vertices[1].y - self.vertices[2].y)
            + self.vertices[1].x * self.vertices[2].y)
    }
}

impl Triangle {
    #[allow(dead_code)]
    pub fn edges(&self) -> [Vector; 3] {
        [
            self.vertices[0].segment_to(&self.vertices[1]),
            self.vertices[1].segment_to(&self.vertices[2]),
            self.vertices[2].segment_to(&self.vertices[0]),
        ]
    }

    #[allow(dead_code)]
    pub fn vertices_to(&self, point: &Point) -> [Vector; 3] {
        [
            self.vertices[0].segment_to(point),
            self.vertices[1].segment_to(point),
            self.vertices[2].segment_to(point),
        ]
    }

    pub fn barycentric_coordinates_from(&self, point: &Point) -> (f64, f64) {
        let s = ((self.vertices[1].y - self.vertices[2].y) * (point.x - self.vertices[2].x)
            + (self.vertices[2].x - self.vertices[1].x) * (point.y - self.vertices[2].y))
            / ((self.vertices[1].y - self.vertices[2].y)
                * (self.vertices[0].x - self.vertices[2].x)
                + (self.vertices[2].x - self.vertices[1].x)
                    * (self.vertices[0].y - self.vertices[2].y));
        let t = ((self.vertices[2].y - self.vertices[0].y) * (point.x - self.vertices[2].x)
            + (self.vertices[0].x - self.vertices[2].x) * (point.y - self.vertices[2].y))
            / ((self.vertices[1].y - self.vertices[2].y)
                * (self.vertices[0].x - self.vertices[2].x)
                + (self.vertices[2].x - self.vertices[1].x)
                    * (self.vertices[0].y - self.vertices[2].y));

        (s, t)
    }

    #[allow(dead_code)]
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

    /// Draw the triangle using flo_canvas, for debug purpose
    pub fn draw(
        &self,
        window_dimension: &(Point, Point),
        canvas: &DrawingTarget,
        line_color: &Color,
    ) {
        canvas.draw(|gc| {
            // Set up the canvas
            gc.canvas_height((window_dimension.1.x - window_dimension.0.x) as f32);
            gc.center_region(
                window_dimension.0.x as f32,
                window_dimension.0.y as f32,
                window_dimension.1.x as f32,
                window_dimension.1.y as f32,
            );

            // Draw a circle
            gc.new_path();
            gc.move_to(self.vertices[0].x as f32, self.vertices[0].y as f32);
            gc.line_to(self.vertices[1].x as f32, self.vertices[1].y as f32);
            gc.line_to(self.vertices[2].x as f32, self.vertices[2].y as f32);
            gc.line_to(self.vertices[0].x as f32, self.vertices[0].y as f32);

            // gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 0.0));
            // gc.fill();
            let width_factor: f32 = 0.001;
            gc.line_width(width_factor * (window_dimension.1.x - window_dimension.0.x) as f32);
            gc.stroke_color(*line_color);
            gc.stroke();
        });
    }

    /// Debug function, prints vertices and adjacencies of a triangle
    pub fn print_triangle(&self) {
        println!(
            "Current_triangle : [({:?}, {:?}), ({:?}, {:?}), ({:?}, {:?})]",
            self.vertices[0].x,
            self.vertices[0].y,
            self.vertices[1].x,
            self.vertices[1].y,
            self.vertices[2].x,
            self.vertices[2].y,
        );
        println!("Adjacencies : {:?}", self.adjacencies);
    }

    /// Computes the circumcircle radius from the triangle
    pub fn circumcircle_radius(&self) -> f64 {
        let mut radius = 0.0;
        for edge in self.edges() {
            radius *= edge.norm();
        }

        radius /= 4.0 * self.signed_area().abs();

        radius
    }

    /// Checks if a point is in the circumcircle of a triangle
    pub fn is_point_in_circumucircle(&self, point: &Point) -> bool {
        let pa = point.segment_to(&self.vertices[0]);
        let pb = point.segment_to(&self.vertices[1]);
        let pc = point.segment_to(&self.vertices[2]);

        //see the circumcircle wikipedia page, this is the determinant of a matrix which will tell if the point is inside or outside of the circumcircle
        // or https://stackoverflow.com/questions/39984709/how-can-i-check-wether-a-point-is-inside-the-circumcircle-of-3-points
        let det = &pa * &pa * (pb.x * pc.y - pc.x * pb.y) - &pb * &pb * (pa.x * pc.y - pc.x * pa.y)
            + &pc * &pc * (pa.x * pb.y - pb.x * pa.y);

        det > 0.0
    }

    /// Finds if the point is one of the triangle's vertices, and returns it index if so. Beware of floating point approwimations.
    pub fn find_point_in_triangle_vertices(&self, point: &Point) -> Option<usize> {
        for i in 0..self.vertices.len() {
            if point == &self.vertices[i] {
                return Some(i);
            }
        }

        None
    }

    /// Finds the face opposite to a triangle vertex
    pub fn find_face_opposite_to(&self, point_local_id: usize) -> Option<Neighbor> {
        self.adjacencies[(point_local_id + 1) % 3]
    }

    /// Finds a point index in a triangle opposite to an edge index
    pub fn find_point_local_id_opposite_to(&self, adjacent_triangle_id: usize) -> Option<usize> {
        for i in 0..3 {
            if self.adjacencies[i] == Some(Neighbor::Cell(adjacent_triangle_id)) {
                return Some((i + 2) % 3);
            }
        }

        None
    }
}

/// Creates a new triangle
pub fn build_triangle(vertices: [Point; 3], adjacencies: [Option<Neighbor>; 3]) -> Triangle {
    Triangle {
        vertices: vertices,
        adjacencies: adjacencies,
    }
}
