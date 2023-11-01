pub mod ray;
pub mod shape;
pub mod vector;

use gl;
use gl::types::*;

use vector::*;
use ray::*;
use crate::three_d::raytracing::shape::{RTObject, RTObjectVec, Sphere};

const aspect_ratio: f32 = 16.0 / 9.0;
pub const image_width: i32 = 1400;

// Calculate the image height, and ensure that it's at least 1.
pub const image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

// Camera

const focal_length: f32 = 1.0;
const viewport_height: f32 = 2.0;
const viewport_width: f32 = viewport_height * image_width as f32 / image_height as f32;

pub fn ray_color(r: &Ray, world: &RTObjectVec) -> Vec3 {
    let rec_wrap = world.ray_intersects(r, 0.0, f32::INFINITY);
    if let Some(rec) = rec_wrap {
        return (rec.normal + Vec3::new([1.0; 3])) * 0.5;
    }

    let unit = r.direction().unit();
    let a = 0.5 * (unit.y() + 1.0);

    Vec3::new([1.0; 3]) * (1.0 - a) + Vec3::new([0.5, 0.7, 1.0]) * a
}

pub fn draw(fb: GLuint, tex: GLuint, dims: (i32, i32), data: &mut Box<[[[f32; 4]; image_width as usize]; image_height as usize]>, world: &RTObjectVec) {
    let camera_center = Vec3::new([0.0, 0.0, 0.0]);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new([viewport_width as f32, 0.0, 0.0]);
    let viewport_v = Vec3::new([0.0, viewport_height, 0.0]);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new([0.0, 0.0, focal_length]) - viewport_u/2 - viewport_v/2;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, world);
            data[j as usize][i as usize][0..3].copy_from_slice(&pixel_color.data());
            data[j as usize][i as usize][3] = 1.0;

        }
    }

    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        //gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
        gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0,
        image_width as GLint, image_height, gl::RGBA, gl::FLOAT,
        data.as_ptr() as *const _);

        gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
        let mut data_buf = vec![0.0_f32; image_width as usize * image_height as usize * 4].into_boxed_slice();
        gl::ReadPixels(0, 0, image_width as GLint, image_height, gl::RGBA, gl::FLOAT, data_buf.as_mut_ptr() as *mut _);

        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fb);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);

        gl::BlitFramebuffer(0, 0, image_width as GLint, image_height, 0, 0, dims.0, dims.1,
            gl::COLOR_BUFFER_BIT, gl::LINEAR);

    }
}
