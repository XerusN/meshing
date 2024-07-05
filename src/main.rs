use std::io;
use std::{thread, time};

use types::*;
use gen_vertices::*;
use triangulation::*;
use flo_canvas::*;
use flo_draw::*;

mod types;
mod gen_vertices;
mod triangulation;

fn main() {
    
    // let point = build_coordinates(0.9, -0.9);
    // let point = build_coordinates(100.0, 100.0);
    // let point = build_coordinates(-100.0, -100.0);
    //let point = build_coordinates(0.9, 0.9);
    // let triangle = build_triangle(None, [
    //     build_coordinates(0.0, 0.0),
    //     build_coordinates(1.0, 0.0),
    //     build_coordinates(0.0, 1.0)
    // ], [None; 3]);
    
    
    
    // for j in -10..11 {
    //     for i in -10..11 {
    //         let point = build_coordinates(i as f64 / 5.0, j as f64 / 5.0);
    //         if triangle.is_point_in_circumucircle(&point) {
    //             print!("x");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    //println!("{:?}", triangle.is_point_in_circumucircle(&point));
    
    with_2d_graphics(|| {
        
        let time_step = time::Duration::from_millis(100);
        
        let vertices = place_vertices(1);
        
        let mut triangles: Vec<Triangle> = Vec::new();
        
        let mut big_triangle = build_triangle(
            None,
            [build_coordinates(-100.0, -100.0), build_coordinates(100.0, 0.0), build_coordinates(0.0, 100.0)],
            [None; 3]
        );
        
        big_triangle.compute_center();
        
        triangles.push(big_triangle.clone());
        
        let mut current_triangle = 0;
        
        let canvas = create_drawing_window("Meshing");
        
        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy).expect("Error in read");
        
        let line_color = Color::Rgba(0.0, 0.0, 0.0, 1.0);
        let window_dimension = (build_coordinates(-10.0, -10.0), build_coordinates(10.0, 10.0));
        //let window_dimension = (build_coordinates(-100.0, -100.0), build_coordinates(100.0, 100.0));
        
        
        for point in &vertices {
            //println!("Current point : ({:?}, {:?})", point.x, point.y);
            current_triangle = find_current_triangle(point, &triangles, current_triangle).expect("No triangle found");
            
            let mut stack = insert_triangles(point, &mut triangles, current_triangle);
            
            // if triangles.len() > 13 {
            //     println!("triangle 9");
            //     triangles[9].print_triangle();
            //     println!();
            //     println!("triangle 5");
            //     triangles[5].print_triangle();
            //     println!();
            //     println!("triangle 12");
            //     triangles[12].print_triangle();
            //     println!();
            //     println!("triangle 13");
            //     triangles[13].print_triangle();
            //     println!();
            // }
            
            canvas.draw(|gc| { gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)) });
            
            for triangle in &triangles {
                triangle.draw(&window_dimension, &canvas, &line_color);
            }
            
            thread::sleep(time_step);
            
            deal_with_delaunay_condition(&mut stack, &mut triangles, point);
            
            canvas.draw(|gc| { gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)) });
            
            for triangle in &triangles {
                triangle.draw(&window_dimension, &canvas, &line_color);
            }
            
            thread::sleep(time_step);
            
            // let mut _dummy = String::new();
            // io::stdin().read_line(&mut _dummy).expect("Error in read");
            
        }
        
        remove_big_triangle(&mut triangles, &big_triangle);
        
        canvas.draw(|gc| { gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)) });
        
        for triangle in &triangles {
            triangle.draw(&window_dimension, &canvas, &line_color);
        }
        
        println!{"Done!"};
    });
}
