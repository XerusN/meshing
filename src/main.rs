use std::io;
use std::time;
use flo_canvas::*;
use flo_draw::*;

use types::*;
use gen_vertices::*;
use triangulation::*;


mod types;
mod gen_vertices;
mod triangulation;

fn main() {
    
    with_2d_graphics(|| {
        
        let time_step = time::Duration::from_millis(100);
        
        let mut vertices = place_vertices(3);
        
        println!("{:?}", vertices.len());
        
        let (min, max) = rescale_vertices(&mut vertices);
        
        let mut triangles: Vec<Triangle> = Vec::new();
        
        let mut big_triangle = build_triangle(
            None,
            [build_point(-100.0, -100.0), build_point(100.0, 0.0), build_point(0.0, 100.0)],
            [None; 3]
        );
        
        big_triangle.compute_center();
        
        triangles.push(big_triangle.clone());
        
        let mut current_triangle = 0;
        
        let canvas = create_drawing_window("Meshing");
        
        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy).expect("Error in read");
        
        let line_color = Color::Rgba(0.0, 0.0, 0.0, 1.0);
        let window_dimension = (min, max);
        //let window_dimension = (build_coordinates(-100.0, -100.0), build_coordinates(100.0, 100.0));
        
        let mut counter = 0;
        
        for point in &vertices {
            //println!("Current point : ({:?}, {:?})", point.x, point.y);
            current_triangle = find_current_triangle(point, &triangles, current_triangle).expect("No triangle found");
            
            let mut stack = insert_triangles(point, &mut triangles, current_triangle);
            
            // canvas.draw(|gc| { gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)) });
            
            // for triangle in &triangles {
            //     triangle.draw(&window_dimension, &canvas, &line_color);
            // }
            
            //thread::sleep(time_step);
            
            deal_with_delaunay_condition(&mut stack, &mut triangles, point);
            
            // canvas.draw(|gc| { gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)) });
            
            // for triangle in &triangles {
            //     triangle.draw(&window_dimension, &canvas, &line_color);
            // }
            
            //thread::sleep(time_step);
            
            println!("{:?}", counter);
            counter += 1;
            
        }
        
        remove_big_triangle(&mut triangles, &big_triangle);
        
        scale_back(&mut vertices, &mut triangles, (&min, &max));
        
        canvas.draw(|gc| { gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)) });
        
        for triangle in &triangles {
            triangle.draw(&window_dimension, &canvas, &line_color);
        }
        
        println!{"Done!"};
    });
}
