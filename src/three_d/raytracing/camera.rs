use std::ffi::c_void;
use std::ptr::slice_from_raw_parts;
use std::slice;
use gl::types::{GLint, GLuint};
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::shape::RTObject;
use crate::three_d::raytracing::vector::Vec3;

pub struct Camera {
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,
    focal_length: f32,
    viewport_width: f32,
    viewport_height: f32,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    saved_dims: (i32, i32),
    data: Box<[f32]>,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, focal_length: f32, viewport_height: f32) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let viewport_width: f32 = viewport_height * image_width as f32 / image_height as f32;
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

        Camera { aspect_ratio, image_width, image_height, focal_length, viewport_width, viewport_height,
            camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v,
            saved_dims: (0, 0),
            data: Box::new([]),
        }
    }
    pub fn render(&mut self, world: &dyn RTObject, fb: GLuint, tex: GLuint, dims: (i32, i32)) {
        // Reallocate data
        if self.saved_dims != dims {
            self.data = vec![0.0_f32; (self.image_height * self.image_width * 4) as usize].into_boxed_slice();
            self.saved_dims = dims;
        }

        // Render
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color = Camera::ray_color(&r, world);
                //let mut vals = unsafe { *data.index(j as usize, i as usize, self.image_width as usize) };
                let idx = (j * self.image_width + i) * 4;
                let vals = &mut self.data[idx as usize ..];
                vals[0..3].copy_from_slice(&pixel_color.data());
                vals[3] = 1.0;
            }
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            //gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0,
                              self.image_width, self.image_height, gl::RGBA, gl::FLOAT,
                              self.data.as_ptr() as *const _);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fb);

            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fb);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);

            gl::BlitFramebuffer(0, 0, self.image_width, self.image_height, 0, 0, dims.0, dims.1,
                                gl::COLOR_BUFFER_BIT, gl::LINEAR);

        }
    }

    fn ray_color(r: &Ray, world: &dyn RTObject) -> Vec3 {
        let rec_wrap = world.ray_intersects(r, Interval::new(0.0, f32::INFINITY));
        if let Some(rec) = rec_wrap {
            return (rec.normal + Vec3::new([1.0; 3])) * 0.5;
        }

        let unit = r.direction().unit();
        let a = 0.5 * (unit.y() + 1.0);

        Vec3::new([1.0; 3]) * (1.0 - a) + Vec3::new([0.5, 0.7, 1.0]) * a
    }
}