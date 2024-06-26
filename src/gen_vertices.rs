use crate::types::*;

pub fn place_vertices(i: i32) -> Vec<Coordinates> {
    
    let mut vertices = Vec::new();
    
    match i {
        0 => {
            vertices.push(build_coordinates(-5.0, 5.0));
            vertices.push(build_coordinates(5.0, 5.0));
            vertices.push(build_coordinates(-2.0, 3.0));
            vertices.push(build_coordinates(3.0, 1.0));
            vertices.push(build_coordinates(-4.0, -1.0));
            vertices.push(build_coordinates(1.0, -2.0));
            vertices.push(build_coordinates(-6.0, -4.0));
            vertices.push(build_coordinates(5.0, -4.0));
        },
        _ => {
            vertices.push(build_coordinates(3.0, 4.0));
            vertices.push(build_coordinates(-2.0, 3.0));
            vertices.push(build_coordinates(-2.0, 2.0));
            vertices.push(build_coordinates(-1.0, -1.0));
            vertices.push(build_coordinates(-2.0, -3.0));
            vertices.push(build_coordinates(4.0, -2.0));
        },
    };
    
    vertices
}