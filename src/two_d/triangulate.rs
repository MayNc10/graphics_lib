use super::shape::{ColorVertex, TextureVertex};

pub fn triangulate_convex_color(vertices: &[ColorVertex]) -> Vec<ColorVertex> {
    let mut shape = Vec::new();
    for i in 2..(vertices.len()) {
        shape.push(vertices[0]);
        shape.push(vertices[i - 1]);
        shape.push(vertices[i]);
    }
    
    shape
}

pub fn triangulate_convex_texture(vertices: &[TextureVertex]) -> Vec<TextureVertex> {
    let mut shape = Vec::new();
    for i in 2..(vertices.len()) {
        shape.push(vertices[0]);
        shape.push(vertices[i - 1]);
        shape.push(vertices[i]);
    }
    
    shape
}

// Do two lines intersect?
pub fn intersects(l1: &([f32; 2], [f32; 2]), l2: &([f32; 2], [f32; 2])) -> bool {
    // we can think of our points as "ranges" in the x and in the y
    // two lines intersect if their x ranegs overlap and if their y ranges overlap
    let range_overlap = |a: (f32, f32), b: (f32, f32)| {
        // Make ranges well formed (x1 < x2)
        let (mut a1, mut a2) = a;
        if a1 > a2 {
            (a2, a1) = a;
        }

        let (mut b1, mut b2) = b;
        if b1 > b2 {
            (b2, b1) = b;
        }
        
        a1 < b2 && b1 < a2
    };


    range_overlap((l1.0[0], l1.1[0]), (l2.0[0], l2.1[0])) 
    && range_overlap((l1.0[1], l1.1[1]), (l2.0[1], l2.1[1])) 
}

// This only works on simple polygons, so nothing with holes
pub fn triangulate_simple_polygon_color(vertices: &[ColorVertex]) -> Vec<ColorVertex> {
    // This is using the ear-clipping method
    // There are faster ways, but this should work fine
    let mut original_vertices = Vec::from(vertices);

    // Get every line from a vertex to its pair
    let mut lines = original_vertices
        .iter() // iterate the vertices
        .skip(1) // Skip the first vertex
        .zip(original_vertices.iter()) // zip them together, so that every pair is of a vertex and it's neighbor
        .map(|a| (a.0.position, a.1.position)) // map color vertices to just position
        .filter(|(a, b)| a != b) // filter any duplicates (should just be last value)
        .collect::<Vec<_>>();


    let mut shape = Vec::new();
    while original_vertices.len() > 3 {
        // loop over all possible indices for starting vertexes
        'start: for i in 0..original_vertices.len() {
            // get the triangle endpoints
            let v1 = original_vertices[i];
            let v3 = original_vertices[i + 2];

            // We want to make a triagle our of i, i + 1, and i + 2
            // For this to always be correct, the line between i and i + 2 can't intersect any of the lines in the current shape
            let l = (v1.position, v3.position);
            for line in &lines {
                if intersects(&l, line) {
                    // skip to the next triplet of vertices
                    continue 'start;
                }
            }
            // Doesn't intersect, these three are good
            // Remove v2 from the original shape
            let v2 = original_vertices.remove(i + 1);
            // Remove the lines from v1 to v2 and v2 to v3
            // But we want then to add a line from v1 to v3
            // So v1 to v2 -> v1 to v3, delete v2 to v3
            lines[i] = (v1.position, v3.position);
            lines.remove(i + 1);

            // Add our new triangle to the shape list
            shape.push(v1);
            shape.push(v2);
            shape.push(v3);
        }
    }
    // We only have three vertices left in the shape, so it's a triangle, add it to Shape
    shape.append(&mut original_vertices);
    shape
}

pub fn triangulate_simple_polygon_texture(vertices: &[TextureVertex]) -> Vec<TextureVertex> {
    // This is using the ear-clipping method
    // There are faster ways, but this should work fine
    let mut original_vertices = Vec::from(vertices);

    // Get every line from a vertex to its pair
    let mut lines = original_vertices
        .iter() // iterate the vertices
        .zip(original_vertices.iter().skip(1)) // zip them together, so that every pair is of a vertex and it's neighbor
        .map(|a| (a.0.position, a.1.position)) // map color vertices to just position
        .filter(|(a, b)| a != b) // filter any duplicates (should just be last value)
        .collect::<Vec<_>>();
    // Add last line, from the last vertex to the first
    lines.push((original_vertices.last().unwrap().position, original_vertices[0].position));

    let mut shape = Vec::new();
    while original_vertices.len() > 3 {
        // loop over all possible indices for starting vertexes
        'start: for i in 0..original_vertices.len() {
            // get the triangle endpoints
            let v1 = original_vertices[i];
            let v3 = original_vertices[i + 2];

            // We want to make a triagle our of i, i + 1, and i + 2
            // For this to always be correct, the line between i and i + 2 can't intersect any of the lines in the current shape
            let l = (v1.position, v3.position);
            for line in &lines {
                if intersects(&l, line) {
                    // skip to the next triplet of vertices
                    continue 'start;
                }
            }
            // Doesn't intersect, these three are good
            // Remove v2 from the original shape
            let v2 = original_vertices.remove(i + 1);
            // Remove the lines from v1 to v2 and v2 to v3
            // But we want then to add a line from v1 to v3
            // So v1 to v2 -> v1 to v3, delete v2 to v3
            lines[i] = (v1.position, v3.position);
            lines.remove(i + 1);

            // Add our new triangle to the shape list
            shape.push(v1);
            shape.push(v2);
            shape.push(v3);

            // Found a good vertex, quit the loop
            break;
        }
    }
    // We only have three vertices left in the shape, so it's a triangle, add it to Shape
    shape.append(&mut original_vertices);
    shape
}