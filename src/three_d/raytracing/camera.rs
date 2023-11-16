use std::ffi::c_void;
use std::ops::Add;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::slice::from_raw_parts;
use std::sync::Arc;
use gl::types::{GLint, GLuint};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use rayon::prelude::*;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::opengl;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::shape::RTObject;
use crate::three_d::raytracing::vector::Vec3;

static INTENSITY: Interval = Interval { min: 0.0, max: 0.999 };

// helper function
fn flatten<T: Copy, const N: usize>(data: &[[T; N]]) -> &[T] {
    unsafe {
        from_raw_parts(data.as_ptr() as *const _, data.len() * N)
    }
}

pub struct Camera {
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,
    focal_length: f32,
    viewport_width: f32,
    viewport_height: f32,
    camera_center: Vec3,

    vfov: f32,
    look_from: Vec3, // Where are we looking from?
    look_at: Vec3, // What point are we looking at?
    vup: Vec3, // What point do we perceive as up?

    // Camera basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    saved_dims: (i32, i32),
    data: Box<[f32]>,

    samples_per_pixel: i32,
    rng: ThreadRng,
    max_depth: i32,

    background: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, look_from: Vec3, look_at: Vec3, vup: Vec3, samples_per_pixel: i32, max_depth: i32, vfov: f32, background: Vec3)
        -> Camera
    {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let focal_length = (look_from - look_at).length();

        let theta = vfov * std::f32::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width: f32 = viewport_height * image_width as f32 / image_height as f32;

        let camera_center = look_from;
        let w = (look_from - look_at).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - (w * focal_length) - viewport_u / 2 - viewport_v / 2;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera { aspect_ratio, image_width, image_height, focal_length, viewport_width, viewport_height,
            camera_center, vfov,
            look_from, look_at, vup, u, v, w,
            pixel00_loc, pixel_delta_u, pixel_delta_v,
            saved_dims: (0, 0),
            data: Box::new([]), samples_per_pixel, rng: thread_rng(), max_depth,
            background
        }
    }
    pub fn render(&mut self, world: &dyn RTObject, dims: (i32, i32), verbose: bool, fb: &mut opengl::Framebuffer) {
        // Reallocate data
        if self.saved_dims != dims {
            self.data = vec![0.0_f32; (self.image_height * self.image_width * 4) as usize].into_boxed_slice();
            self.saved_dims = dims;
        }

        // Render
        for j in 0..self.image_height {
            let current_idx = (j * self.image_width) as f32;
            let max_idx = (self.image_height * self.image_width) as f32;

            if verbose { print!("\rProgress: {}%, idx: {} out of {}", current_idx/max_idx * 100.0, current_idx as u32, max_idx as u32); }

            for i in 0..self.image_width {


                let mut pixel_color = Vec3::new([0.0; 3]);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world, &mut self.rng, self.max_depth, self.background);
                }

                pixel_color /= self.samples_per_pixel;
                let clamping: fn(f32) -> f32 = |num| INTENSITY.clamp(num);
                pixel_color.for_each(&clamping);
                let gamma_correction: fn(f32) -> f32 = |num| Camera::correct_gamma(num);
                pixel_color.for_each(&gamma_correction);

                //let mut vals = unsafe { *data.index(j as usize, i as usize, self.image_width as usize) };
                let idx = (j * self.image_width + i) * 4;
                let vals = &mut self.data[idx as usize ..];
                vals[0..3].copy_from_slice(&pixel_color.data());
                vals[3] = 1.0;
            }
        }

        unsafe { fb.draw(self.image_width, self.image_height, dims, self.data.as_ptr()); }
    }

    pub fn render_parallel(&mut self, world: &dyn RTObject, dims: (i32, i32), verbose: bool, fb: &mut opengl::Framebuffer) {
        // Reallocate data
        if self.saved_dims != dims {
            self.data = vec![0.0_f32; (self.image_height * self.image_width * 4) as usize].into_boxed_slice();
            self.saved_dims = dims;
        }
        // Make world cross-thread
        let arc_world = Arc::new(world);
        let data_ptr = self.data.as_mut_ptr() as usize;

        let idx_iter = (0..(self.image_height * self.image_width)).into_par_iter();
        idx_iter.for_each(|idx| {
            let j = idx / self.image_width;
            let i = idx % self.image_width;

            // Collect samples in parallel as well
            let samples_iter = (0..self.samples_per_pixel).into_par_iter();
            let mut pixel_color: Vec3 = samples_iter.map(|_| {
                let r = Camera::get_ray_cmethod(i, j, self.pixel00_loc, self.pixel_delta_u, self.pixel_delta_v, self.camera_center);
                Camera::ray_color_parallel(&r, arc_world.clone(), self.max_depth, self.background)
            }).sum();

            pixel_color /= self.samples_per_pixel;
            let clamping: fn(f32) -> f32 = |num| INTENSITY.clamp(num);
            pixel_color.for_each(&clamping);
            let gamma_correction: fn(f32) -> f32 = |num| Camera::correct_gamma(num);
            pixel_color.for_each(&gamma_correction);

            // This is *really* dangerous, but it should work
            unsafe {
                let data_ptr = (data_ptr as *mut f32).add(idx as usize * 4);
                *data_ptr = pixel_color.x();
                *data_ptr.add(1) = pixel_color.y();
                *data_ptr.add(2) = pixel_color.z();
                *data_ptr.add(3) = 1.0;
            }
        });

        unsafe { fb.draw(self.image_width, self.image_height, dims, self.data.as_ptr()); }
    }


    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    fn get_ray_cmethod(i: i32, j: i32, pixel00_loc: Vec3, pixel_delta_u: Vec3, pixel_delta_v: Vec3, camera_center: Vec3) -> Ray {
        let pixel_center = pixel00_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
        let pixel_sample = pixel_center + Camera::pixel_sample_square_cmethod(pixel_delta_u, pixel_delta_v);

        let ray_direction = pixel_sample - camera_center;
        Ray::new(camera_center, ray_direction)
    }

    fn pixel_sample_square(&mut self) -> Vec3 {
        let px = -0.5 + self.rng.gen::<f32>();
        let py = -0.5 + self.rng.gen::<f32>();

        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn pixel_sample_square_cmethod(pixel_delta_u: Vec3, pixel_delta_v: Vec3) -> Vec3 {
        let mut rng = thread_rng();

        let px = -0.5 + rng.gen::<f32>();
        let py = -0.5 + rng.gen::<f32>();

        pixel_delta_u * px + pixel_delta_v * py
    }

    fn ray_color(r: &Ray, world: &dyn RTObject, rng: &mut ThreadRng, depth: i32, background: Vec3) -> Vec3 {
        // Don't gather more light if we've reached the depth
        if depth <= 0 { return Vec3::new([0.0; 3]) }

        // Ignore hits that are too close, they are probably from "shadow acne"
        let rec_wrap = world.ray_intersects(r, Interval::new(0.001, f32::INFINITY));
        if let Some(rec) = rec_wrap {
            return {
                if let Some((attenuation, scattered)) = rec.mat.scatter(*r, rec.self_without_mat()) {
                    attenuation * Camera::ray_color(&scattered, world, rng, depth - 1, background) + rec.mat.emitted()
                } else { rec.mat.emitted() }
            };
        }

        background
    }

    fn ray_color_parallel(r: &Ray, world: Arc<&dyn RTObject>, depth: i32, background: Vec3) -> Vec3 {
        let mut rng = thread_rng();
        Camera::ray_color(r, *world, &mut rng, depth, background)
    }

    fn correct_gamma(x: f32) -> f32 { x.sqrt() }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(16.0 / 9.0, 1400,
                    Vec3 { data: [0.0, 0.0, -1.0] }, Vec3 { data: [0.0; 3] }, Vec3 { data: [0.0, 1.0, 0.0] },
            10, 10, 90.0, Vec3::default())
    }
}