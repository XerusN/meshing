use crate::types::base::point::*;

/// Creates a point cloud, for now only for test purpose
pub fn place_vertices(i: i32) -> Vec<Point> {
    let mut vertices = Vec::new();

    match i {
        0 => {
            vertices.push(build_point(-5.0, 5.0));
            vertices.push(build_point(5.0, 5.0));
            vertices.push(build_point(-2.0, 3.0));
            vertices.push(build_point(3.0, 1.0));
            vertices.push(build_point(-4.0, -1.0));
            vertices.push(build_point(1.0, -2.0));
            vertices.push(build_point(-6.0, -4.0));
            vertices.push(build_point(5.0, -4.0));
        }
        1 => {
            vertices.push(build_point(3.0, 4.0));
            vertices.push(build_point(-2.0, 3.0));
            vertices.push(build_point(-2.0, 2.0));
            vertices.push(build_point(-1.0, -1.0));
            vertices.push(build_point(-2.0, -3.0));
            vertices.push(build_point(4.0, -2.0));
        }
        2 => {
            let resolution = 100;
            let (l_x, l_y) = (2.0, 1.0);
            for i in 0..resolution {
                for j in 0..resolution {
                    vertices.push(build_point(
                        (i as f64 * l_x) / (resolution as f64 - 1.0),
                        (j as f64 * l_y) / (resolution as f64 - 1.0),
                    ))
                }
            }
        }
        _ => {
            let resolution = 100;
            let (l_x, l_y) = (2.0, 1.0);
            let circle_center = build_point(1.0, 0.0);
            let circle_radius = 0.25;
            for i in 0..resolution {
                for j in 0..resolution {
                    let (x, y) = (
                        (i as f64 * l_x) / (resolution as f64 - 1.0),
                        (j as f64 * l_y) / (resolution as f64 - 1.0),
                    );
                    if (x - circle_center.x) * (x - circle_center.x)
                        + (y - circle_center.y) * (y - circle_center.y)
                        >= circle_radius * circle_radius
                    {
                        vertices.push(build_point(x, y));
                    }
                }
            }
        }
    };

    vertices
}
