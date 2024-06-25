//use types::{build_coordinates, Coordinates};
use types::*;
use flo_canvas::*;
use flo_draw::*;

mod types;
mod gen_vertices;

fn main() {
    
    
    
    with_2d_graphics(|| {
        
        let a = build_coordinates(0.0, 0.0);
        let b = build_coordinates(1.0, 0.0);
        let c = build_coordinates(0.0, 1.0);
        
        let triangle = types::Triangle{
            center : None,
            vertices : [a, b, c],
            adjacencies : [None, None, None],
        };
        
        let canvas = create_drawing_window("Meshing");
        
        let line_color = Color::Rgba(0.0, 0.0, 0.0, 1.0);
        let window_dimension = (build_coordinates(-1.0, -1.0), build_coordinates(2.0, 2.0));
        
        triangle.draw(&window_dimension, &canvas, &line_color)
    });
}
