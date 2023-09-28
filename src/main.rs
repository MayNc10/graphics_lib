use glutin::{*, event_loop::EventLoop};
use gl::types::*;
use graphics_lib::three_d::VAO::VAOLock;
use graphics_lib::three_d::VAO::VertexArrayObject;
use graphics_lib::three_d::shape;
use graphics_lib::three_d::shape::Light;
use graphics_lib::three_d::shape::Shape;
use graphics_lib::three_d::shape::Transform;
use graphics_lib::three_d::shape::importing;
use std::ffi::CString;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::str;

use graphics_lib::three_d::shaders::{*, self};
//use graphics_lib::{three_d::shape::Light};
use graphics_lib::three_d;
//use graphics_lib::three_d::animation;
use graphics_lib::matrix::*;
//use graphics_lib::three_d::teapot;
use image;
use graphics_lib::three_d::buffer::*;

// Set a target for fps (don't run faster or slower than this)
const TARGET_FPS: u64 = 60;

fn main() {
    /* 
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));
    
    //demo_2d(event_loop, display);
    demo_3d(event_loop, gl_window);
    */
    demo_3d();
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
        

        // Draw, using the vertexs, the shaders, and the matrix
        shape.draw(&mut target, &display);
        target.finish().unwrap();
    });
}
*/

//static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
static VERTEX_DATA: [Vertex; 4] = [
    [0.0, 0.0, 0.0], 
    [0.5, 0.0, 0.0], 
    [0.0, 0.5, 0.0],
    [0.5, 0.5, 0.0]];
static INDICES: [GLuint; 6] = [0, 1, 2, 1, 2, 3];

// Shader sources
static VS_SRC: &'static str = "
#version 150
in vec3 position;

void main() {
    gl_Position = vec4(position, 1.0);
}";

static FS_SRC: &'static str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

//event_loop: EventLoop<()>, gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>

fn buffers(lock: &VAOLock, vertices: &[Vertex], indices: &[GLuint]) -> (VertexBuffer, IndexBuffer) {
    let v_buffer = VertexBuffer::new(vertices, &lock);
    let index_buffer = IndexBuffer::new(indices, &lock);

    (v_buffer, index_buffer)
}

fn demo_3d() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    // Create GLSL shaders
    let vs = compile_shader(shaders::BLINN_PHONG_3D_SHADER, shaders::ShaderProgramType::Vertex);
    let fs = compile_shader(shaders::BLINN_PHONG_3D_FRAG_SHADER, shaders::ShaderProgramType::Fragment);
    let program = link_program(vs, fs);

    let angle_func = |t: f32| { (t / 5.0) * 360.0 };
    let rotation_animation = 
        Box::new(three_d::animation::Rotation {ty: three_d::animation::RotationType::X, angle_func}) as Box<dyn three_d::animation::Animation>;  

    let mut s = Shape::from_obj(
        "media\\torus.obj", 
        ShaderType::BlinnPhong, 
        None, 
        Some(rotation_animation), 
        false, 
    ).unwrap();

    println!("Torus Material: {:?}", s.material);


    unsafe { 
        // Use shader program
        gl::UseProgram(program.0); 
        gl::BindFragDataLocation(program.0, 0, CString::new("color").unwrap().as_ptr());

        s.bind_attributes(&program);
    }

    s.set_scaling(generate_scale(&[0.5; 3]));
    s.set_translation(generate_translate(None, None, Some(2.0)));

    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();


    let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

    event_loop.run(move |event, _, control_flow| {
        

        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // Cleanup
                    unsafe {
                        gl::DeleteProgram(program.0);
                        gl::DeleteShader(fs.0);
                        gl::DeleteShader(vs.0);
                    }
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

                    s.animate(t);

                    let dims = gl_window.window().inner_size();
                    let dims = (dims.width as f32, dims.height as f32);

                    let light = Light {
                        direction: [1.0, 1.0, -1.0],

                        ambient: [1.0, 1.0, 1.0],
                        diffuse: [1.0, 1.0, 1.0],
                        specular: [1.0, 1.0, 1.0],
                    };

                    unsafe {
                        // Clear the screen to black
                        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }

                    s.draw(&light, &view, &program, dims);      
                    
                    gl_window.swap_buffers().unwrap();

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
                *control_flow =  glutin::event_loop::ControlFlow::WaitUntil(new_inst);
                //println!("Hitting fps goal!");
            }
        }
    });
}

/*
// Create programs

let light = Light {
        direction: [1.0; 3],

        ambient: [0.05; 3],
        diffuse: [1.0; 3],
        specular: [1.0; 3],
    };
    let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

    let mut scene = three_d::scene::Scene::new(view, light);

    let angle_func = |t: f32| { (t / 5.0) * 360.0 };
    let rotation_animation = 
        Box::new(animation::Rotation {ty: animation::RotationType::Y, angle_func}) as Box<dyn animation::Animation>;  

    let mut cube = three_d::shape::Shape::from_obj("media\\square.obj", 
        three_d::shaders::ShaderType::None, 
        None, 
        Some(rotation_animation), 
        false).unwrap().pop().unwrap();

    cube.set_scaling(generate_scale(&[0.3; 3]));
    cube.set_translation(generate_translate(Some(0.5), None, Some(2.0)));
    scene.add_shape(cube);


    // t is our start time, delta is what we increase it by each time
    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    // Create the main event loop
    
    event_loop.run(move |event, _, control_flow| {
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
            glutin::event::Event::RedrawRequested(_) => {}
            glutin::event::Event::RedrawEventsCleared => {}
            _ => return,
        }

        // How long has this pass taken?
        let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

        // How long should we wait for to run at 60 fps?
        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0
        };
        if wait_millis != 0 {
            let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
            // Wait that long
            *control_flow =  glutin::event_loop::ControlFlow::WaitUntil(new_inst);
            //println!("Hitting fps goal!");
        }
        else {
            // Update time
            t += delta;

            // Create a drawing target
            unsafe {
                gl::ClearColor(0.0, 0.6, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }            

            let size = gl_window.window().inner_size();
            scene.draw(t, (size.height as f32, size.width as f32));
            gl_window.swap_buffers().unwrap();

            start_time = std::time::Instant::now();
        }
    });

*/