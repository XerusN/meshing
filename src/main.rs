use std::io;

use types::*;
use gen_vertices::*;
use triangulation::*;
use flo_canvas::*;
use flo_draw::*;

mod types;
mod gen_vertices;
mod triangulation;

fn main() {
    
    with_2d_graphics(|| {
        
        let vertices = place_vertices(0);
        
        let mut triangles: Vec<Triangle> = Vec::new();
        
        let mut big_triangle = build_triangle(
            None,
            [build_coordinates(-100.0, -100.0), build_coordinates(100.0, 0.0), build_coordinates(0.0, 100.0)],
            [None; 3]
        );
        
        big_triangle.compute_center();
        
        triangles.push(big_triangle);
        
        let mut current_triangle = 0;
        
        let canvas = create_drawing_window("Meshing");
        
        let line_color = Color::Rgba(0.0, 0.0, 0.0, 1.0);
        let window_dimension = (build_coordinates(-10.0, -10.0), build_coordinates(10.0, 10.0));
        //let window_dimension = (build_coordinates(-100.0, -100.0), build_coordinates(100.0, 100.0));
        
        
        for point in &vertices {
            println!("Current point : ({:?}, {:?})", point.x, point.y);
            //println!("{:?}", triangles[0].include(&point));
            current_triangle = find_current_triangle(point, &triangles, current_triangle).expect("No triangle found");
            println!("Current_triangle : {:?}", current_triangle);
            
        //     println!("Current_triangle : [({:?}, {:?}), ({:?}, {:?}), ({:?}, {:?})]",
        //     triangles[current_triangle].vertices[0].x,
        //     triangles[current_triangle].vertices[0].y,
        //     triangles[current_triangle].vertices[1].x,
        //     triangles[current_triangle].vertices[1].y,
        //     triangles[current_triangle].vertices[2].x,
        //     triangles[current_triangle].vertices[2].y,
        // );
            
            let stack = insert_triangles(point, &mut triangles, current_triangle);
            println!("New triangles : {:?}", stack);
            
            
            canvas.draw(|gc| { gc.clear() });
            
            for triangle in &triangles {
                triangle.draw(&window_dimension, &canvas, &line_color);
            }
            
            let mut _dummy = String::new();
            io::stdin().read_line(&mut _dummy).expect("Error in read");
            
        }
        
        println!{"Done!"};
    });
}
