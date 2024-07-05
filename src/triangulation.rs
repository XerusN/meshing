use crate::types::*;
use core::panic;
use std::io;

pub fn find_current_triangle(point: &Coordinates, triangles: &Vec<Triangle>, last_triangle_index: usize) -> Option<usize> {
    
    let mut i = last_triangle_index;
    let mut result: Option<usize> = None;
    
    
    // Not working yet, wip. But should be broken no matter what when implementing the constrined version of the triangulation
    // let mut old1;
    // let mut old2 = -1;
    
    // loop {
        
    //     if i < triangles.len() {
            
    //         println!("{:?}", i);
            
    //         if triangles[i].include(&point) {
    //             result = Some(i);
    //             break;
    //         } else {
    //             old1 = i as i64;
                
    //             i = triangles[i].adjacencies[triangles[i].find_face_to_point(point, old2).expect("No face found to the point in find_current_triangle")].expect("No adjacent triangle for this face in find_current_triangle");
    //             if old2 == i as i64 {
    //                 panic!("i = old");
    //             }
    //             old2 = old1;
    //         }

    //     } else {
    //         break;
    //     }
    // }
    
    i = 0;
    
    loop {
        
        if i < triangles.len() {
            
            //println!("{:?}", i);
            
            if triangles[i].include(&point) {
                result = Some(i);
                break;
            } else {
                i += 1;
            }

        } else {
            panic!("Point not found in any vertices");
        }
    }
    
    result
    
}

pub fn insert_triangles(point: &Coordinates, triangles : &mut Vec<Triangle>, current_triangle: usize) -> Vec<usize> {
    
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
    
    new_triangles
    
}

pub fn deal_with_delaunay_condition(stack : &mut Vec<usize>, triangles : &mut Vec<Triangle>, point: &Coordinates) {
    
    if stack.is_empty() {
        panic!("stack is empty");
    }
    
    loop {
        
        // println!("Stack : {:?}", stack);
        // let mut _dummy = String::new();
        // io::stdin().read_line(&mut _dummy).expect("Error in read");
        
        let triangle_id = match stack.pop() {
            None => {
                //println!("Empty stack");
                break;
            },
            Some(id) => id,
        };
        
        let triangle = triangles[triangle_id].clone();
    
        let point_local_id = match triangle.find_point_in_triangle(point) {
            None => panic!("Triangle not containing current point on the stack"),
            Some(id) => id,
        };
        
        let (opposite_triangle, opposite_triangle_id) = match triangle.find_face_opposite_to(point_local_id) {
            None => continue,   //No edge swap needed
            Some(id) => (triangles[id].clone(), id),
        };
        
        //If point is not is circumcircle no edge swap is needed
        if !opposite_triangle.is_point_in_circumucircle(point) {
            continue;
        }
        
        let opposite_point_local_id = match opposite_triangle.find_point_local_id_opposite_to(triangle_id) {
            None => panic!("source triangle is not adjacent to the opposite? FF"),
            Some(id) => id,
        };
        
        let new_triangle_1 = Triangle {
            center: None,
            vertices: [point.clone(), opposite_triangle.vertices[opposite_point_local_id].clone(), triangle.vertices[(point_local_id + 2) % 3].clone()],
            adjacencies: [Some(opposite_triangle_id), opposite_triangle.adjacencies[opposite_point_local_id], triangle.adjacencies[(point_local_id + 2) % 3]],
        };
        
        match opposite_triangle.adjacencies[opposite_point_local_id] {
            None => (),
            Some(id) => {
                for i in 0..triangles[id].adjacencies.len() {
                    if triangles[id].adjacencies[i] == Some(opposite_triangle_id) {
                        triangles[id].adjacencies[i] = Some(triangle_id);
                    }
                }
            },
        }
        
        let new_triangle_2 = Triangle {
            center: None,
            vertices: [point.clone(), triangle.vertices[(point_local_id + 1) % 3].clone(), opposite_triangle.vertices[opposite_point_local_id].clone()],
            adjacencies: [triangle.adjacencies[point_local_id], opposite_triangle.adjacencies[(opposite_point_local_id + 2) % 3], Some(triangle_id)],
        };
        
        match triangle.adjacencies[point_local_id] {
            None => (),
            Some(id) => {
                for i in 0..triangles[id].adjacencies.len() {
                    if triangles[id].adjacencies[i] == Some(triangle_id) {
                        triangles[id].adjacencies[i] = Some(opposite_triangle_id);
                    }
                }
            },
        }
        
        triangles[triangle_id] = new_triangle_1;
        triangles[opposite_triangle_id] = new_triangle_2;
        
        if !stack.contains(&triangle_id) {
            stack.push(triangle_id);
        }
        if !stack.contains(&opposite_triangle_id) {
            stack.push(opposite_triangle_id);
        }
        
    }
}

pub fn remove_big_triangle(triangles: &mut Vec<Triangle>, big_triangle: &Triangle) {
    
    let mut i = 0;
    
    loop {
        
        if i >= triangles.len() {
            break;
        }
        
        if (big_triangle.find_point_in_triangle(&triangles[i].vertices[0]) != None) |
            (big_triangle.find_point_in_triangle(&triangles[i].vertices[1]) != None) |
            (big_triangle.find_point_in_triangle(&triangles[i].vertices[2]) != None)
        {
            for k in 0..triangles.len() {
                for j in 0..3 {
                    if triangles[k].adjacencies[j] == Some(i) {
                        triangles[k].adjacencies[j] = None;
                    } else if triangles[k].adjacencies[j] == Some(triangles.len() - 1) {
                        triangles[k].adjacencies[j] = Some(i);
                    }
                }
            }
            triangles.swap_remove(i);
            i -= 1;
        }
        
        i += 1;
        
    }
    
}