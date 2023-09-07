pub type Matrix = [[f32; 4]; 4];

// Define an identity matrix
pub const IDENTITY: Matrix = [
    [1.0, 0.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0, 0.0], 
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0f32],
];

pub fn generate_transform(angle: Option<f32>, x_offset: Option<f32>, y_offset: Option<f32>, scaling: Option<&[f32; 3]>) -> Matrix {
    let angle = angle.unwrap_or(0.0);
    let x_offset = x_offset.unwrap_or(0.0);
    let y_offset = y_offset.unwrap_or(0.0);

    let scaling = scaling.unwrap_or(&[1.0; 3]);

    let transform = [
        [scaling[0] * angle.cos(), scaling[0] * angle.sin(), 0.0, 0.0], 
        [scaling[1] * -angle.sin(), scaling[1] * angle.cos(), 0.0, 0.0], 
        [0.0, 0.0, scaling[2], 0.0],
        [x_offset, y_offset, 0.0, 1.0f32],
    ];

    return transform;
}

