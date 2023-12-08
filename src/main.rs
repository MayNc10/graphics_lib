use glutin::event_loop::EventLoop;
use lazy_static::lazy_static;
use clap::Parser;
use graphics_lib::three_d::buffer::FrameBuffer;
use graphics_lib::three_d::scene::{Scene, init_deferred_quad};
use graphics_lib::three_d::lights::{DirectionLight, PointLight};
use graphics_lib::three_d::shape::{Shape, Transform};

use std::ffi::CString;
use std::path::Path;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use glutin::dpi::PhysicalSize;
use rand::{random, Rng, thread_rng};
use graphics_lib::cli::Cli;

use graphics_lib::three_d::shaders::{*, self};
use graphics_lib::three_d;
use graphics_lib::matrix::*;
use graphics_lib::three_d::raytracing::bvh::BVHNode;
use graphics_lib::three_d::raytracing::camera::Camera;
use graphics_lib::three_d::raytracing::lights::DiffuseLight;
use graphics_lib::three_d::raytracing::material::{Dielectric, Lambertian, Metal};
use graphics_lib::three_d::raytracing::{opengl, random_cosine_direction};
use graphics_lib::three_d::raytracing::pdf::SpherePDF;
use graphics_lib::three_d::raytracing::shape::{make_box, Quad, RotateY, RTObject, RTObjectVec, Sphere, Translate, Tri};
use graphics_lib::three_d::raytracing::shape::polyhedron::Polyhedron;
use graphics_lib::three_d::raytracing::vector::Vec3;

// Set a target for fps (don't run faster or slower than this)
const TARGET_FPS: u64 = 100;
const ASPECT_RATIO: f32 = 1.92;
const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const FOCAL_LENGTH: f32 = 1.0;
const VFOV: f32 = 80.0;
const LOOK_FROM: Vec3 = Vec3 { data: [0.0, 0.0, 8.0] };
const LOOK_AT: Vec3 = Vec3 { data: [0.0, 0.0, 0.0] };
const VUP: Vec3 = Vec3 { data: [0.0, 1.0, 0.0] };
const SAMPLES_PER_PIXEL: i32 = 35;
const MAX_DEPTH: i32 = 15;

const BACKGROUND: Vec3 = Vec3 { data: [0.0, 0.0, 0.0] };

lazy_static! {
    static ref MEDIA_PATH_BASE: &'static Path = Path::new("media");
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let window: glutin::window::WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("Graphics Lib")
        .with_inner_size(PhysicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT));
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    demo_rt_parallelogram(gl_window);

    let window: glutin::window::WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("Graphics Lib")
        .with_inner_size(PhysicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT));
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    demo_rt_circles(gl_window);

    let window: glutin::window::WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("Graphics Lib")
        .with_inner_size(PhysicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT));
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    demo_rt_cornell(gl_window);

    let window: glutin::window::WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("Graphics Lib")
        .with_inner_size(PhysicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT));
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    demo_rt_glass_shards(gl_window);

    let window: glutin::window::WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("Graphics Lib")
        .with_inner_size(PhysicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT));
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    demo_rt_cubes(gl_window);
}

/* 
fn demo_2d(event_loop: EventLoop<()>, display: Display) {
    // List vertices for our "lighting bolt"
    /* 
    let vertex1 = two_d::shape::TextureVertex { position: [0.0, 0.0], tex_coords: [0.5, 0.5] };
    let vertex2 = two_d::shape::TextureVertex { position: [0.0, 1.0], tex_coords: [0.0, 1.0] };
    let vertex3 = two_d::shape::TextureVertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0] };
    let vertex4 = two_d::shape::TextureVertex { position: [1.0, 0.0], tex_coords: [1.0, 0.5] };
    let vertex5 = two_d::shape::TextureVertex { position: [0.5, 0.0], tex_coords: [0.75, 0.5] };
    let vertex6 = two_d::shape::TextureVertex { position: [0.5, -1.0], tex_coords: [0.75, 0.0] };
    let vertex7 = two_d::shape::TextureVertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] };
    let vertex8 = two_d::shape::TextureVertex { position: [-1.0, 0.0], tex_coords: [0.0, 0.5] };
    */
    let vertex1 = two_d::shape::TextureVertex { position: [-1.0, 1.0], tex_coords: [0.0, 1.0] };
    let vertex2 = two_d::shape::TextureVertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0] };
    let vertex3 = two_d::shape::TextureVertex { position: [1.0, -1.0], tex_coords: [1.0, 0.0] };
    let vertex4 = two_d::shape::TextureVertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] };


    let image_bytes = include_bytes!("..\\media\\dargenio.jpg");

    // Rotate some angle
    let mut angle: f32 = 0.0;
    angle = (angle / 180.0) * std::f32::consts::PI;
    let transform = generate_rotate_x(angle);

    // Create a shape from the vertices. We list the vertices in such a way to create two triangles, since triangles are the primitive two_d::shape
    let mut shape = two_d::shape::Shape::new_convex_texture(
        &[vertex1, vertex2, vertex3, vertex4], 
            &display, image_bytes, image::ImageFormat::Jpeg, Some(&transform));

    // Set a target for fps (don't run faster or slower than this)
    const TARGET_FPS: u64 = 60;

    // t is our start time, delta is what we increase it by each time
    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    // Create the main event loop
    event_loop.run(move |event, _, control_flow| {
        // When did this pass start?
        let start_time = std::time::Instant::now();

        // Handle window closing events (return) and New events from the OS (return or ignore)
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        // How long has this pass taken?
        let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

        // How long should we wait for to run at 60 fps?
        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0
        };
        let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
        // Wait that long
        *control_flow =  glutin::event_loop::ControlFlow::WaitUntil(new_inst);
        // Update time
        t += delta;

        // Create a drawing target
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // animate
        
        if true { //t < 5.0 {
            let scale = [t / 5.0; 3];
            let angle = (360.0 * (t/ 2.0)) / 180.0 * std::f32::consts::PI;

            let transform = generate_scale(&scale) * generate_rotate_x(angle);

            shape.set_transform_matrix(transform);

        } 
        

        // Draw, using the vertices, the shaders, and the matrix
        shape.draw(&mut target, &display);
        target.finish().unwrap();
    });
}
*/

fn demo_rt_circles(gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();

    let ground_mat = Arc::new(Lambertian::new(Vec3::new([0.5; 3])));
    world.add(Box::new(Sphere::new( [0.0, -1000.0, 0.0].into(), 1000.0, ground_mat.clone() )) );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vec3::new([a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>()]);

            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vec3::random(&mut thread_rng()) * Vec3::random(&mut thread_rng());
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                world.add(Box::new( Sphere::new( center, 0.2, sphere_mat ) ));

            } else if choose_mat < 0.95  {
                // metal
                let albedo = Vec3::random_in_range(&mut thread_rng(), 0.5..=1.0);
                let fuzz = thread_rng().gen_range(0.0..=0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                world.add(Box::new( Sphere::new( center, 0.2, sphere_mat ) ));
            } else {
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                world.add(Box::new( Sphere::new( center, 0.2, sphere_mat ) ));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new( Vec3::new([0.0, 1.0, 0.0]), 1.0, mat1) ));

    let mat2 = Arc::new(Lambertian::new(Vec3::new([0.4, 0.2, 0.1])));
    world.add(Box::new(Sphere::new( Vec3::new([-4.0, 1.0, 0.0]), 1.0, mat2) ));

    let mat3 = Arc::new(Metal::new(Vec3::new([0.7, 0.6, 0.5]), 0.0));
    world.add(Box::new(Sphere::new( Vec3::new([4.0, 1.0, 0.0]), 1.0, mat3) ));

    let world = BVHNode::from(world);
    //let world = RTObjectVec::new_from_vec(vec![Box::new(world)]);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [13.0, 2.0, 3.0].into(), [0.0; 3].into(), VUP,
                            cli.samples_per_pixel.unwrap_or(5000), cli.max_depth.unwrap_or(500),
                                 cli.vfov.unwrap_or(20.0), [0.5, 0.7, 1.0].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, None, dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, None, dims, cli.verbose, &mut fb);
    }

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("circles.png")).to_str().unwrap());

    println!("Done Rendering!");

}

fn demo_rt_cornell(gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();

    let red = Arc::new(Lambertian::new([0.65, 0.05, 0.05].into()));
    let white = Arc::new(Lambertian::new([0.73, 0.73, 0.73].into()));
    let green = Arc::new(Lambertian::new([0.12, 0.45, 0.15].into()));
    let light = Arc::new(DiffuseLight::new([15.0; 3].into()));


    world.add(Box::new(Quad::new([555.0, 0.0, 0.0].into(), [0.0, 555.0, 0.0].into(), [0.0, 0.0, 555.0].into(), green, false)));
    world.add(Box::new(Quad::new([0.0; 3].into(), [0.0, 555.0, 0.0].into(), [0.0, 0.0, 555.0].into(), red, false)));
    world.add(Box::new(Quad::new([343.0, 554.0, 332.0].into(), [-130.0, 0.0, 0.0].into(), [0.0, 0.0, -105.0].into(), light.clone(), false)));
    world.add(Box::new(Quad::new([0.0; 3].into(), [555.0, 0.0, 0.0].into(), [0.0, 0.0, 555.0].into(), white.clone(), false)));
    world.add(Box::new(Quad::new([555.0; 3].into(), [-555.0, 0.0, 0.0].into(), [0.0, 0.0, -555.0].into(), white.clone(), false)));
    world.add(Box::new(Quad::new([0.0, 0.0, 555.0].into(), [555.0, 0.0, 0.0].into(), [0.0, 555.0, 0.0].into(), white.clone(), false)));

    let box1 =make_box([0.0;3].into(), [165.0, 330.0, 165.0].into(), white.clone());
    let box1 = Box::new(RotateY::new(box1, 15.0));
    let box1 = Box::new(Translate::new(box1, [265.0, 0.0, 295.0].into()));
    world.add(box1);

    //let box2 = make_box([0.0; 3].into(), [165.0; 3].into(), white.clone());
    //let box2 = Box::new(RotateY::new(box2, -18.0));
    //let box2 = Box::new(Translate::new(box2, [130.0, 0.0, 65.0].into()));
    //world.add(box2);
    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new([190.0, 90.0, 190.0].into(), 90.0, glass)));

    let mut lights = RTObjectVec::new();
    lights.add(Box::new(Quad::new([343.0, 554.0, 332.0].into(), [-130.0, 0.0, 0.0].into(), [0.0, 0.0, -105.0].into(), light, false)));

    let world = BVHNode::from(world);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [278.0, 278.0, -800.0].into(), [278.0, 278.0, 0.0].into(), VUP,
                                 cli.samples_per_pixel.unwrap_or(5000), cli.max_depth.unwrap_or(500),
                                 cli.vfov.unwrap_or(40.0), [0.0; 3].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, Some(&lights), dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, Some(&lights), dims, cli.verbose, &mut fb);
    }

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("cornell_boxes.png")).to_str().unwrap());

    println!("Done Rendering!");
}

fn demo_rt_parallelogram(gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();

    let red = Arc::new(Lambertian::new([0.65, 0.05, 0.05].into()));
    let white = Arc::new(Lambertian::new([0.73, 0.73, 0.73].into()));
    let glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([1.0; 3]) * 3.0));
    let green = Arc::new(Lambertian::new([0.12, 0.45, 0.15].into() ));
    let green_glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([0.12, 0.45, 0.15]) * (3.0 / 0.45) ));
    let light = Arc::new(DiffuseLight::new([30.0; 3].into()));

    //world.add(Box::new(Quad::new([10.0, 0.0, 0.0].into(), [0.0, 10.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone())));
    //world.add(Box::new(Quad::new([0.0; 3].into(), [0.0, 10.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone())));

    let light_quad = Box::new(Quad::new([-1.0, -2.99, 2.0].into(), [2.0, 0.0, 0.0].into(), [0.0, 0.0, -2.0].into(), light.clone(), false));
    world.add(light_quad.clone_dyn());

    let light_quad_2 = Box::new(Quad::new([1.0, 2.99, 2.0].into(), [-2.0, 0.0, 0.0].into(), [0.0, 0.0, -2.0].into(), light.clone(), false));
    world.add(light_quad_2.clone_dyn());

    world.add(Box::new(Quad::new([-5.0, 3.0, 5.0].into(), [0.0, -6.0, 0.0].into(), [0.0, 0.0, -10.0].into(), glass.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, -3.0, 5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 0.0, -10.0].into(), green_glass.clone(), false)));
    world.add(Box::new(Quad::new([5.0, -3.0, 5.0].into(), [0.0, 6.0, 0.0].into(), [0.0, 0.0, -10.0].into(), glass.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, 3.0, 5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, -3.0, -5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 10.0, 0.0].into(), red.clone(), false)));

    //let t1 = Box::new(Tri::new([[0.0, 1.0, -1.0].into(), [-1.0, 0.0, -1.0].into(), [1.0, 0.0, -1.0].into()], [0.0, 0.0, 1.0].into(), red.clone()));
    //world.add(t1);
    let p1 = Box::new(Translate::new(Box::new(Polyhedron::from_obj(MEDIA_PATH_BASE.join("parallelogram.obj").to_str().unwrap()).unwrap()),
                                     [0.0, 0.0, 0.0].into()) );
    world.add(p1);

    let mut lights = RTObjectVec::new();
    lights.add(light_quad);
    lights.add(light_quad_2);

    let world = BVHNode::from(world);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [0.0, 0.0, 8.0].into(), [0.0; 3].into(), VUP,
                                 cli.samples_per_pixel.unwrap_or(5000), cli.max_depth.unwrap_or(600),
                                 cli.vfov.unwrap_or(80.0), [0.0; 3].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, Some(&lights), dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, Some(&lights), dims, cli.verbose, &mut fb);
    }

    gl_window.swap_buffers().unwrap();
    println!("Finished rendering!");

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("parallelogram.png")).to_str().unwrap());
}

fn demo_rt_glass_shards(gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();

    let red = Arc::new(Lambertian::new([0.90625, 0.21875, 0.21875].into()));
    let orange = Arc::new(Lambertian::new([0.746, 0.2578, 0.1328].into()));
    let white = Arc::new(Lambertian::new([0.73, 0.73, 0.73].into()));
    let black = Arc::new(Lambertian::new([0.0; 3].into()));
    let grey = Arc::new(Lambertian::new([0.5; 3].into()));
    let glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([1.0; 3]) * 3.0));
    let green = Arc::new(Lambertian::new([0.12, 0.45, 0.15].into() ));
    let blue = Arc::new(Lambertian::new([0.1328, 0.289, 0.746].into() ));
    let purple = Arc::new(Lambertian::new([0.5625, 0.21875, 0.90625].into() ));
    let blue_metal = Arc::new(Metal::new([0.0078, 0.14, 0.43].into(), 0.0));
    let metal = Arc::new(Metal::new([1.0; 3].into(), 0.0));
    let green_glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([0.12, 0.45, 0.15]) * (3.0 / 0.45) ));
    let cyan = Arc::new(Lambertian::new([0.0, 1.0, 1.0].into() ));
    let yellow = Arc::new(Lambertian::new([1.0, 1.0, 0.0].into() ));
    let magenta = Arc::new(Lambertian::new([1.0, 0.0, 1.0].into() ));
    let light = Arc::new(DiffuseLight::new([30.0; 3].into()));

    //world.add(Box::new(Quad::new([10.0, 0.0, 0.0].into(), [0.0, 10.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone())));
    //world.add(Box::new(Quad::new([0.0; 3].into(), [0.0, 10.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone())));

    let light_quad = Box::new(Quad::new([-1.0, -2.99, 2.0].into(), [2.0, 0.0, 0.0].into(), [0.0, 0.0, -2.0].into(), light.clone(), false));
    world.add(light_quad.clone_dyn());

    let light_quad_2 = Box::new(Quad::new([1.0, 2.99, 2.0].into(), [-2.0, 0.0, 0.0].into(), [0.0, 0.0, -2.0].into(), light.clone(), false));
    //world.add(light_quad_2.clone_dyn());

    world.add(Box::new(Quad::new([-5.0, 3.0, 5.0].into(), [0.0, -6.0, 0.0].into(), [0.0, 0.0, -10.0].into(), cyan.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, -3.0, 5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 0.0, -10.0].into(), grey.clone(), false)));
    world.add(Box::new(Quad::new([5.0, -3.0, 5.0].into(), [0.0, 6.0, 0.0].into(), [0.0, 0.0, -10.0].into(), yellow.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, 3.0, 5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 0.0, -10.0].into(), grey.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, -3.0, -5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 10.0, 0.0].into(), magenta.clone(), false)));


    // create random glass shards
    let num_shards = 150;

    let max_side_length = 1.0;
    let min_side_length = 0.5;

    let mut rng = thread_rng();
    for i in 0..num_shards {
        // pick random point in -2 to 2
        let random_x = rng.gen::<f32>() * 8.0 - 4.0;
        let random_y = rng.gen::<f32>() * 5.0 - 2.5;
        let random_z = rng.gen::<f32>() * 8.0 - 4.0;

        let p1: Vec3 = [random_x, random_y, random_z].into();

        // pick random directions
        let rand_dir_1 = random_cosine_direction() * (rng.gen::<f32>() * (max_side_length - min_side_length) + min_side_length);
        let rand_dir_2 = random_cosine_direction() * (rng.gen::<f32>() * (max_side_length - min_side_length) + min_side_length);

        let p2 = p1 + rand_dir_1;
        let p3 = p1 + rand_dir_2;

        let tri = Tri::new([p1, p2, p3], Vec3::cross(&rand_dir_1, &rand_dir_2).unit(), metal.clone());
        world.add(Box::new(tri));
    }

    let mut lights = RTObjectVec::new();
    lights.add(light_quad);
    //lights.add(light_quad_2);

    let world = BVHNode::from(world);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [0.0, 0.0, 8.0].into(), [0.0; 3].into(), VUP,
                                 cli.samples_per_pixel.unwrap_or(1000), cli.max_depth.unwrap_or(400),
                                 cli.vfov.unwrap_or(80.0), [0.0; 3].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, Some(&lights), dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, Some(&lights), dims, cli.verbose, &mut fb);
    }

    gl_window.swap_buffers().unwrap();
    println!("Finished rendering!");

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("glass_shards.png")).to_str().unwrap());

}

fn demo_rt_cubes(gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();

    let red = Arc::new(Lambertian::new([0.90625, 0.21875, 0.21875].into()));
    let orange = Arc::new(Lambertian::new([0.746, 0.2578, 0.1328].into()));
    let white = Arc::new(Lambertian::new([0.73, 0.73, 0.73].into()));
    let black = Arc::new(Lambertian::new([0.0; 3].into()));
    let grey = Arc::new(Lambertian::new([0.5; 3].into()));
    let glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([1.0; 3]) * 3.0));
    let green = Arc::new(Lambertian::new([0.242, 0.5078, 0.2539].into() ));
    let blue = Arc::new(Lambertian::new([0.1328, 0.289, 0.746].into() ));
    let purple = Arc::new(Lambertian::new([0.5625, 0.21875, 0.90625].into() ));
    let blue_metal = Arc::new(Metal::new([0.0078, 0.14, 0.43].into(), 0.0));
    let metal = Arc::new(Metal::new([1.0; 3].into(), 0.0));
    let green_glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([0.12, 0.45, 0.15]) * (3.0 / 0.45) ));
    let cyan = Arc::new(Lambertian::new([0.0, 1.0, 1.0].into() ));
    let yellow = Arc::new(Lambertian::new([1.0, 1.0, 0.0].into() ));
    let magenta = Arc::new(Lambertian::new([1.0, 0.0, 1.0].into() ));
    let light = Arc::new(DiffuseLight::new([10.0; 3].into()));

    let light_sphere = Box::new( Sphere::new([0.0, 0.25, 0.0].into(), 0.5, light.clone()) );
    world.add(light_sphere.clone());


    // create cubesS
    let cube_size = 3.0;
    let num_cubes_along_axis = 20;
    let start_coord = Vec3::new( [-cube_size, 0.0, -cube_size] ) * (num_cubes_along_axis as f32 / 2.0);

    let y_level = 6.0; // the level for the floor and ceiling
    let random_shift_lim = 3.0;

    let mut rng = thread_rng();

    for x_cube_idx in 0..num_cubes_along_axis {
        for z_cube_idx in 0..num_cubes_along_axis {

            let b = if rng.gen() { make_box([0.0; 3].into(), [cube_size; 3].into(), green.clone()) }
                else { make_box([0.0; 3].into(), [cube_size; 3].into(), green_glass.clone()) };

            let random_y_variation = rng.gen::<f32>() - 0.5 * 2.0 * random_shift_lim;
            let mut upper_pos: Vec3 = start_coord + Vec3::new( [0.0, y_level, 0.0] );
            upper_pos += [cube_size * x_cube_idx as f32, random_y_variation, cube_size * z_cube_idx as f32].into();

            let b_upper = Translate::new(b, upper_pos);
            world.add(Box::new(b_upper));

            let b = if rng.gen() { make_box([0.0; 3].into(), [cube_size; 3].into(), green.clone()) }
            else { make_box([0.0; 3].into(), [cube_size; 3].into(), green_glass.clone()) };

            let random_y_variation = rng.gen::<f32>() - 0.5 * 2.0 * random_shift_lim;
            let mut lower_pos: Vec3 = start_coord - Vec3::new( [0.0, y_level, 0.0] );
            lower_pos += [cube_size * x_cube_idx as f32, random_y_variation, cube_size * z_cube_idx as f32].into();

            let b_lower = Translate::new(b.clone(), lower_pos);
            world.add(Box::new(b_lower));
        }
    }


    let mut lights = RTObjectVec::new();
    lights.add(light_sphere);

    let world = BVHNode::from(world);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [0.0, 0.0, 10.0].into(), [0.0; 3].into(), VUP,
                                 cli.samples_per_pixel.unwrap_or(50), cli.max_depth.unwrap_or(15),
                                 cli.vfov.unwrap_or(80.0), [0.0; 3].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, Some(&lights), dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, Some(&lights), dims, cli.verbose, &mut fb);
    }

    gl_window.swap_buffers().unwrap();
    println!("Finished rendering!");

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("cubes.png")).to_str().unwrap());

}

fn demo_rt_chair(event_loop: EventLoop<()>, gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();

    let red = Arc::new(Lambertian::new([0.65, 0.05, 0.05].into()));
    let white = Arc::new(Lambertian::new([0.73, 0.73, 0.73].into()));
    let glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([1.0; 3]) * 3.0));
    let green = Arc::new(Lambertian::new([0.12, 0.45, 0.15].into() ));
    let green_glass = Arc::new(Dielectric::new_colored(1.5, Vec3::new([0.12, 0.45, 0.15]) * (3.0 / 0.45) ));
    let light = Arc::new(DiffuseLight::new([30.0; 3].into()));

    //world.add(Box::new(Quad::new([10.0, 0.0, 0.0].into(), [0.0, 10.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone())));
    //world.add(Box::new(Quad::new([0.0; 3].into(), [0.0, 10.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone())));


    let light_quad_2 = Box::new(Quad::new([1.0, 2.99, 2.0].into(), [-2.0, 0.0, 0.0].into(), [0.0, 0.0, -2.0].into(), light.clone(), false));
    world.add(light_quad_2.clone_dyn());

    world.add(Box::new(Quad::new([-5.0, 3.0, 5.0].into(), [0.0, -6.0, 0.0].into(), [0.0, 0.0, -10.0].into(), green.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, -3.0, 5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone(), false)));
    world.add(Box::new(Quad::new([5.0, -3.0, 5.0].into(), [0.0, 6.0, 0.0].into(), [0.0, 0.0, -10.0].into(), green.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, 3.0, 5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 0.0, -10.0].into(), white.clone(), false)));
    world.add(Box::new(Quad::new([-5.0, -3.0, -5.0].into(), [10.0, 0.0, 0.0].into(), [0.0, 10.0, 0.0].into(), red.clone(), false)));

    //let t1 = Box::new(Tri::new([[0.0, 1.0, -1.0].into(), [-1.0, 0.0, -1.0].into(), [1.0, 0.0, -1.0].into()], [0.0, 0.0, 1.0].into(), red.clone()));
    //world.add(t1);
    let p1 = Box::new(Translate::new(Box::new(Polyhedron::from_obj(MEDIA_PATH_BASE.join("chair.obj").to_str().unwrap()).unwrap()),
                                     [0.0, -2.0, 0.0].into()) );
    world.add(p1);

    let mut lights = RTObjectVec::new();
    lights.add(light_quad_2);

    let world = BVHNode::from(world);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [0.0, 0.0, 8.0].into(), [0.0; 3].into(), VUP,
                                 cli.samples_per_pixel.unwrap_or(SAMPLES_PER_PIXEL), cli.max_depth.unwrap_or(MAX_DEPTH),
                                 cli.vfov.unwrap_or(80.0), [0.0; 3].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, Some(&lights), dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, Some(&lights), dims, cli.verbose, &mut fb);
    }

    gl_window.swap_buffers().unwrap();
    println!("Finished rendering!");

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("chair.png")).to_str().unwrap());

    event_loop.run(move |event, _, control_flow| {

        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::MouseInput { .. } => {
                    gl_window.swap_buffers().unwrap();
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

                // How long should we wait for to run at 60 fps?
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };

                if wait_millis == 0 {
                    // Update time
                    t += delta;

                    //gl_window.swap_buffers().unwrap();

                    start_time = std::time::Instant::now();
                }

            },
            _ => (),
        }

        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                gl_window.window().request_redraw();

                let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

                // How long should we wait for to run at 60 fps?
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };

                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                // Wait that long
                *control_flow =  ControlFlow::WaitUntil(new_inst);
                //println!("Hitting fps goal!");
            }
        }
    });
}

fn demo_rt(event_loop: EventLoop<()>, gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let cli = Cli::parse();

    let dims_ps = gl_window.window().inner_size();
    let dims = (dims_ps.width as i32, dims_ps.height as i32);

    let image_width = cli.image_width.unwrap_or(IMAGE_WIDTH);
    let image_height = (image_width as f32 / cli.aspect_ratio.unwrap_or(ASPECT_RATIO)) as i32;

    let mut fb = opengl::Framebuffer::new(image_width, image_height);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut world = RTObjectVec::new();


    //let t1 = Box::new(Tri::new([[0.0, 1.0, -1.0].into(), [-1.0, 0.0, -1.0].into(), [1.0, 0.0, -1.0].into()], [0.0, 0.0, 1.0].into(), red.clone()));
    //world.add(t1);
    let p1 = Box::new(Translate::new(Box::new(Polyhedron::from_obj(MEDIA_PATH_BASE.join("square.obj").to_str().unwrap()).unwrap()),
                                     [0.0, 0.0, 0.0].into()) );
    world.add(p1);


    let world = BVHNode::from(world);


    let mut camera = Camera::new(cli.aspect_ratio.unwrap_or(ASPECT_RATIO), image_width,
                                 [0.0, 0.0, 3.0].into(), [0.0; 3].into(), VUP,
                                 cli.samples_per_pixel.unwrap_or(50), cli.max_depth.unwrap_or(20),
                                 cli.vfov.unwrap_or(80.0), [1.0, 0.0, 0.0].into());

    // Render once
    unsafe {
        gl::BindBuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    }

    if cli.no_parallel {
        camera.render(&world, None, dims, cli.verbose, &mut fb);
    } else {
        camera.render_parallel(&world, None, dims, cli.verbose, &mut fb);
    }

    gl_window.swap_buffers().unwrap();
    println!("Finished rendering!");

    camera.draw_to_image(MEDIA_PATH_BASE.join(Path::new("square.png")).to_str().unwrap());

    event_loop.run(move |event, _, control_flow| {

        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::MouseInput { .. } => {
                    gl_window.swap_buffers().unwrap();
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

                // How long should we wait for to run at 60 fps?
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };

                if wait_millis == 0 {
                    // Update time
                    t += delta;

                    //gl_window.swap_buffers().unwrap();

                    start_time = std::time::Instant::now();
                }

            },
            _ => (),
        }

        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                gl_window.window().request_redraw();

                let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

                // How long should we wait for to run at 60 fps?
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };

                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                // Wait that long
                *control_flow =  ControlFlow::WaitUntil(new_inst);
                //println!("Hitting fps goal!");
            }
        }
    });
}



fn shape_path(name: &str) -> String {
    String::from(MEDIA_PATH_BASE.join(Path::new(name)).to_str().unwrap())
}

fn demo_3d(event_loop: EventLoop<()>, gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }



    let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

    let light = DirectionLight {
        direction: [-1.0, -1.0, -1.0],

        ambient: [0.05, 0.05, 0.05],
        diffuse: [0.4, 0.4, 0.4],
        specular: [0.5, 0.5, 0.5],
    };

    let point_light = PointLight {
        position: [-1.0, 1.0, -1.0], //MISNOMER IS POSITION

        ambient: [0.05, 0.05, 0.05],
        diffuse: [0.8, 0.8, 0.8],
        specular: [1.0, 1.0, 1.0],

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    };

    let point_light_2 = PointLight {
        position: [1.0, 1.0, -1.0], //MISNOMER IS POSITION

        ambient: [0.05, 0.05, 0.05],
        diffuse: [0.8, 0.8, 0.8],
        specular: [1.0, 1.0, 1.0],

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    };

    init_deferred_quad();

    let dims = gl_window.window().inner_size();
    let width = dims.width as i32;
    let height = dims.height as i32;

    let mut frame_buffer = FrameBuffer::new(width, height);

    // Create GLSL shaders
    let program = &*shaders::BLINN_PHONG;

    let prepass_program = &*shaders::PREPASS;

    let lighting_program = &*shaders::BLINN_PHONG_LIGHTING;

    let point_lighting_program = &*shaders::BLINN_PHONG_POINT_LIGHTING;

    let emission = &*shaders::EMISSION;

    let mut scene = Scene::new(view, program, vec![light], vec![]);

    let color_name = CString::new("color").unwrap();

    lighting_program.bind_color_output(&color_name);
    point_lighting_program.bind_color_output(&color_name);
    program.bind_color_output(&color_name);

    let angle_func = |t: f32| { (-t / 5.0) * 360.0 };
    let rotation_animation = 
        Box::new(three_d::animation::Rotation {ty: three_d::animation::RotationType::X, angle_func}) as Box<dyn three_d::animation::Animation>;  

    let mut s = Shape::from_obj(
        &shape_path("torus.obj"),
        ShaderType::BlinnPhong,
        None,
        Some(rotation_animation),
        false, 
    ).unwrap();

    s.bind_attributes(&program);

    s.set_scaling(generate_scale(&[0.33; 3]));
    s.set_translation(generate_translate(Some(-0.5), None, Some(-2.0)));

    scene.add_shape(s);


    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();
    let mut dims_ps = gl_window.window().inner_size();
    let mut dims = (dims_ps.width as f32, dims_ps.height as f32);

    event_loop.run(move |event, _, control_flow| {
        
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {  
                let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

                // How long should we wait for to run at 60 fps?
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };

                if wait_millis == 0 {
                    // Update time
                    t += delta;

                    if dims_ps != gl_window.window().inner_size() {
                        dims_ps = gl_window.window().inner_size();
                        dims = (dims_ps.width as f32, dims_ps.height as f32);
                        frame_buffer = FrameBuffer::new(dims.0 as i32, dims.0 as i32);
                    }

                    scene.draw_deferred(t, dims, &gl_window, 
                        prepass_program, lighting_program, point_lighting_program, emission,
                        &frame_buffer
                    );

                    start_time = std::time::Instant::now(); 
                } 

            },
            _ => (),
        }

        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                gl_window.window().request_redraw();

                let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

                // How long should we wait for to run at 60 fps?
                let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                    true => 1000 / TARGET_FPS - elapsed_time,
                    false => 0
                };

                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                // Wait that long
                *control_flow =  ControlFlow::WaitUntil(new_inst);
                //println!("Hitting fps goal!");
            }
        }
    });
}