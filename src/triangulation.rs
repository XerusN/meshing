use crate::types::*;

use core::panic;

/// Rescale all vertices to make everything fit in [0, 1].
/// Ensures for exemple the the big triangle will always be big enough.
///
/// Returns the original length of the point cloud to be able to scale it back
pub fn rescale_vertices(vertices: &mut Vec<Point>) -> (Point, Point) {
    let (mut x_max, mut x_min) = (vertices[0].x, vertices[0].x);
    let (mut y_max, mut y_min) = (vertices[0].y, vertices[0].y);

    for vertex in &*vertices {
        if vertex.x > x_max {
            x_max = vertex.x;
        } else if vertex.x < x_min {
            x_min = vertex.x;
        }
        if vertex.y > y_max {
            y_max = vertex.y;
        } else if vertex.y < y_min {
            y_min = vertex.y;
        }
    }

    let scale_factor = if x_max - x_min > y_max - y_min {
        x_max - x_min
    } else {
        y_max - y_min
    };

    for vertex in &mut *vertices {
        vertex.x = (vertex.x - x_min) / scale_factor;
        vertex.y = (vertex.y - y_min) / scale_factor;
    }

    (build_point(x_min, y_min), build_point(x_max, y_max))
}

/// Scales the point cloud back to its original size (changed by rescale_vertices)
pub fn scale_back(
    vertices: &mut Vec<Point>,
    triangles: &mut Vec<Triangle>,
    (min, max): (&Point, &Point),
) {
    let scale_factor = if max.x - min.x > max.y - min.y {
        max.x - min.x
    } else {
        max.y - min.y
    };

    for vertex in &mut *vertices {
        vertex.x = vertex.x * scale_factor + min.x;
        vertex.y = vertex.y * scale_factor + min.y;
    }

    for triangle in &mut *triangles {
        for i in 0..triangle.vertices.len() {
            triangle.vertices[i].x = triangle.vertices[i].x * scale_factor + min.x;
            triangle.vertices[i].y = triangle.vertices[i].y * scale_factor + min.y;
        }
    }
}

/// Finds the cell in which the current point is.
/// For now only a bruteforce (but not very efficient) algorithm is implemented.
pub fn find_current_cell<T: Cell>(point: &Point, mesh: &Vec<T>) -> Option<usize> {
    let mut i = 0;
    let mut result;

    loop {
        if i < mesh.len() {
            if mesh[i].include(&point) {
                result = Some(i);
                break;
            } else {
                i += 1;
            }
        } else {
            result = None;
        }
    }

    result
}

/// Inserts the new triangles created for the new vertex.
pub fn insert_triangles(
    point: &Point,
    triangles: &mut Vec<Triangle>,
    current_triangle: usize,
) -> Vec<usize> {
    let old_triangle = triangles[current_triangle].clone();
    let mut new_triangles = Vec::new();

    new_triangles.push(current_triangle);
    new_triangles.push(triangles.len());
    new_triangles.push(triangles.len() + 1);

    triangles[current_triangle] = build_triangle(
        [
            old_triangle.vertices[0],
            old_triangle.vertices[1],
            point.clone(),
        ],
        [
            old_triangle.adjacencies[0],
            Some(Neighbor::Cell(new_triangles[1])),
            Some(Neighbor::Cell(new_triangles[2])),
        ],
    );

    triangles.push(build_triangle(
        [
            old_triangle.vertices[1],
            old_triangle.vertices[2],
            point.clone(),
        ],
        [
            old_triangle.adjacencies[1],
            Some(Neighbor::Cell(new_triangles[2])),
            Some(Neighbor::Cell(new_triangles[0])),
        ],
    ));

    triangles.push(build_triangle(
        [
            old_triangle.vertices[2],
            old_triangle.vertices[0],
            point.clone(),
        ],
        [
            old_triangle.adjacencies[2],
            Some(Neighbor::Cell(new_triangles[0])),
            Some(Neighbor::Cell(new_triangles[1])),
        ],
    ));

    //CHECK HOW IT WORKS SINCE NEIGHBOR CHANGE
    for i in 0..3 {
        match triangles[new_triangles[i]].adjacencies[0] {
            None => (),
            Some(neighbor) => match neighbor {
                Neighbor::Boundary => (),
                Neighbor::Cell(l) => {
                    for j in 0..3 {
                        match triangles[l].adjacencies[j] {
                            None => (),
                            Some(k) => match k {
                                Neighbor::Boundary => (),
                                Neighbor::Cell(m) => {
                                    if m == current_triangle {
                                        triangles[l].adjacencies[j] =
                                            Some(Neighbor::Cell(new_triangles[i]))
                                    }
                                }
                            },
                        }
                    }
                }
            },
        };
    }

    new_triangles
}

/// Corrects the mesh after new triangle insertion to ensure Delaunay condition remains true.
pub fn deal_with_delaunay_condition(
    stack: &mut Vec<usize>,
    triangles: &mut Vec<Triangle>,
    point: &Point,
) {
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
            }
            Some(id) => id,
        };

        let triangle = triangles[triangle_id].clone();

        let point_local_id = match triangle.find_point_in_triangle_vertices(point) {
            None => panic!("Triangle not containing current point on the stack"),
            Some(id) => id,
        };

        let (opposite_triangle, opposite_triangle_id) =
            match triangle.find_face_opposite_to(point_local_id) {
                None => continue, //No edge swap needed
                Some(neighbor) => match neighbor {
                    Neighbor::Cell(id) => (triangles[id].clone(), id),
                    Neighbor::Boundary => continue,
                },
            };

        // If point is not is circumcircle no edge swap is needed
        if !opposite_triangle.is_point_in_circumucircle(point) {
            continue;
        }

        let opposite_point_local_id =
            match opposite_triangle.find_point_local_id_opposite_to(triangle_id) {
                None => panic!("source triangle is not adjacent to the opposite? FF"),
                Some(id) => id,
            };

        let new_triangle_1 = Triangle {
            vertices: [
                point.clone(),
                opposite_triangle.vertices[opposite_point_local_id].clone(),
                triangle.vertices[(point_local_id + 2) % 3].clone(),
            ],
            adjacencies: [
                Some(Neighbor::Cell(opposite_triangle_id)),
                opposite_triangle.adjacencies[opposite_point_local_id],
                triangle.adjacencies[(point_local_id + 2) % 3],
            ],
        };

        match opposite_triangle.adjacencies[opposite_point_local_id] {
            None => (),
            Some(neighbor) => match neighbor {
                Neighbor::Boundary => (),
                Neighbor::Cell(id) => {
                    for i in 0..triangles[id].adjacencies.len() {
                        if triangles[id].adjacencies[i]
                            == Some(Neighbor::Cell(opposite_triangle_id))
                        {
                            triangles[id].adjacencies[i] = Some(Neighbor::Cell(triangle_id));
                        }
                    }
                }
            },
        }

        let new_triangle_2 = Triangle {
            vertices: [
                point.clone(),
                triangle.vertices[(point_local_id + 1) % 3].clone(),
                opposite_triangle.vertices[opposite_point_local_id].clone(),
            ],
            adjacencies: [
                triangle.adjacencies[point_local_id],
                opposite_triangle.adjacencies[(opposite_point_local_id + 2) % 3],
                Some(Neighbor::Cell(triangle_id)),
            ],
        };

        match triangle.adjacencies[point_local_id] {
            None => (),
            Some(neighbor) => match neighbor {
                Neighbor::Boundary => (),
                Neighbor::Cell(id) => {
                    for i in 0..triangles[id].adjacencies.len() {
                        if triangles[id].adjacencies[i] == Some(Neighbor::Cell(triangle_id)) {
                            triangles[id].adjacencies[i] =
                                Some(Neighbor::Cell(opposite_triangle_id));
                        }
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

/// The big triangle created at the start is no more needed
pub fn remove_big_triangle(triangles: &mut Vec<Triangle>, big_triangle: &Triangle) {
    let mut i = 0;

    loop {
        if i >= triangles.len() {
            break;
        }

        if (big_triangle.find_point_in_triangle_vertices(&triangles[i].vertices[0]) != None)
            | (big_triangle.find_point_in_triangle_vertices(&triangles[i].vertices[1]) != None)
            | (big_triangle.find_point_in_triangle_vertices(&triangles[i].vertices[2]) != None)
        {
            for k in 0..triangles.len() {
                for j in 0..3 {
                    if triangles[k].adjacencies[j] == Some(Neighbor::Cell(i)) {
                        triangles[k].adjacencies[j] = None;
                    } else if triangles[k].adjacencies[j]
                        == Some(Neighbor::Cell(triangles.len() - 1))
                    {
                        triangles[k].adjacencies[j] = Some(Neighbor::Cell(i));
                    }
                }
            }
            triangles.swap_remove(i);
        } else {
            i += 1;
        }
    }
}
