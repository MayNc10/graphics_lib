use glutin::event_loop::EventLoop;
use lazy_static::lazy_static;
use graphics_lib::three_d::buffer::FrameBuffer;
use graphics_lib::three_d::scene::{Scene, init_deferred_quad};
use graphics_lib::three_d::lights::{DirectionLight, PointLight};
use graphics_lib::three_d::shape::Shape;

use std::ffi::CString;
use std::path::Path;

use graphics_lib::three_d::shaders::{*, self};
use graphics_lib::three_d;
use graphics_lib::matrix::*;

// Set a target for fps (don't run faster or slower than this)
const TARGET_FPS: u64 = 60;
lazy_static! {
    static ref MEDIA_PATH_BASE: &'static Path = Path::new("media");
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window: glutin::window::WindowBuilder = glutin::window::WindowBuilder::new()
        .with_title("Graphics Lib");
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    demo_3d(event_loop, gl_window);
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

    let frame_buffer = FrameBuffer::new(width, height);

    // Create GLSL shaders
    let program = &*shaders::BLINN_PHONG;

    let prepass_program = &*shaders::PREPASS;

    let lighting_program = &*shaders::BLINN_PHONG_LIGHTING;

    let point_lighting_program = &*shaders::BLINN_PHONG_POINT_LIGHTING;

    let emission = &*shaders::EMISSION;

    let mut scene = Scene::new(view, program, vec![light], vec![point_light, point_light_2]);

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

    let rotation_animation = 
        Box::new(three_d::animation::Rotation {ty: three_d::animation::RotationType::Z, angle_func}) as Box<dyn three_d::animation::Animation>;  

    let mut monkey = Shape::from_obj(
        &shape_path("monkey.obj"),
        ShaderType::BlinnPhong,
        None,
        Some(rotation_animation),
        false, 
    ).unwrap();

    monkey.bind_attributes(&program);

    monkey.set_scaling(generate_scale(&[0.33; 3]));
    monkey.set_translation(generate_translate(Some(0.5), None, Some(-2.0)));

    scene.add_shape(monkey);

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

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

                    let dims = gl_window.window().inner_size();
                    let dims = (dims.width as f32, dims.height as f32);  

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