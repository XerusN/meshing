use crate::types::*;

pub fn find_current_triangle(point: &Coordinates, triangles: &Vec<Triangle>, last_triangle_index: usize) -> Option<usize> {
    
    let mut i = last_triangle_index;
    let mut result: Option<usize> = None;
    
    let mut old1;
    let mut old2 = -1;
    
    loop {
        
        if i < triangles.len() {
            
            println!("{:?}", i);
            
            println!("Current_triangle : [({:?}, {:?}), ({:?}, {:?}), ({:?}, {:?})]",
                triangles[i].vertices[0].x,
                triangles[i].vertices[0].y,
                triangles[i].vertices[1].x,
                triangles[i].vertices[1].y,
                triangles[i].vertices[2].x,
                triangles[i].vertices[2].y,
            );
            
            if triangles[i].include(&point) {
                result = Some(i);
                break;
            } else {
                old1 = i as i64;
                
                i = triangles[i].adjacencies[triangles[i].find_face_to_point(point).expect("No face found to the point in find_current_triangle")].expect("No adjacent triangle for this face in find_current_triangle");
                if old2 == i as i64 {
                    panic!("i = old");
                }
                old2 = old1;
            }

        } else {
            break;
        }
    }
    
    // i = 0;
    
    // loop {
        
    //     if i < triangles.len() {
            
    //         println!("{:?}", i);
            
    //         if triangles[i].include(&point) {
    //             result = Some(i);
    //             break;
    //         } else {
    //             i += 1;
    //         }

    //     } else {
    //         break;
    //     }
    // }
    
    result
    
}

pub fn insert_triangles(point: &Coordinates, triangles : &mut Vec<Triangle>, current_triangle: usize) -> Option<Vec<usize>> {
    
    let old_triangle = triangles[current_triangle].clone();
    let mut new_triangles = Vec::new();
    
    new_triangles.push(current_triangle);
    new_triangles.push(triangles.len());
    new_triangles.push(triangles.len() + 1);
    
    triangles[current_triangle] = build_triangle(None, [old_triangle.vertices[0], old_triangle.vertices[1], point.clone()], [old_triangle.adjacencies[0], Some(new_triangles[1]), Some(new_triangles[2])]);
    
    triangles.push(build_triangle(None, [old_triangle.vertices[1], old_triangle.vertices[2], point.clone()], [old_triangle.adjacencies[1], Some(new_triangles[2]), Some(new_triangles[0])]));
    
    triangles.push(build_triangle(None, [old_triangle.vertices[2], old_triangle.vertices[0], point.clone()], [old_triangle.adjacencies[2], Some(new_triangles[0]), Some(new_triangles[1])]));
    
    for i in &new_triangles {
        triangles[*i].compute_center();
    }
    
    for i in 0..3 {
        match triangles[new_triangles[i]].adjacencies[0] {
            None => (),
            _ => {
                for j in 0..3 {
                    if triangles[triangles[new_triangles[i]].adjacencies[0].expect("No value in adjacency when should have been checked")].adjacencies[j] == Some(current_triangle) {
                        let index = triangles[new_triangles[i]].adjacencies[0].expect("No value in adjacency when should have been checked");
                        triangles[index].adjacencies[j] = Some(new_triangles[i]);
                        break;
                    }
                }
            },
        };
    }
    
    Some(new_triangles)
    
}

pub fn deal_with_stack(stack : Vec<usize>, triangles : &mut Vec<Triangle>) {
    
    //
    
}