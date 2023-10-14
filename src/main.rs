use glutin::event_loop::EventLoop;
use gl::types::*;
use graphics_lib::three_d::scene::Scene;
use graphics_lib::three_d::lights::{DirectionLight, PointLight};
use graphics_lib::three_d::shape::Shape;


use std::ffi::CString;
use std::mem;
use std::ptr;

use graphics_lib::three_d::shaders::{*, self};
use graphics_lib::three_d;
use graphics_lib::matrix::*;

// Set a target for fps (don't run faster or slower than this)
const TARGET_FPS: u64 = 60;

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
        

        // Draw, using the vertexs, the shaders, and the matrix
        shape.draw(&mut target, &display);
        target.finish().unwrap();
    });
}
*/

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


    let mut quad_vao = 0;
    let mut quad_vbo = 0;
    unsafe {
        let quad_vertices = [
            // positions        // texture Coords
            -1.0,  1.0, 0.0,
            -1.0, -1.0, 0.0,
            1.0,  1.0, 0.0, 
            1.0, -1.0, 0.0_f32,
        ];

        // setup plane VAO
        gl::GenVertexArrays(1, &mut quad_vao);
        gl::GenBuffers(1, &mut quad_vbo);
        gl::BindVertexArray(quad_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&quad_vertices) as isize, &quad_vertices[0] as *const f32 as *const GLvoid, 
            gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as i32, 
            ptr::null());
    }

    let dims = gl_window.window().inner_size();
    let dims = (dims.width as f32, dims.height as f32);  

    let mut g_buffer = 0;
    let mut g_position = 0;
    let mut g_normal = 0;
    let mut g_color_diffuse = 0;
    let mut g_color_emission = 0;
    let mut g_color_specular = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut g_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, g_buffer);
        
        // - position color buffer
        gl::GenTextures(1, &mut g_position);
        gl::BindTexture(gl::TEXTURE_2D, g_position);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32, dims.0 as i32, dims.1 as i32, 0, 
            gl::RGBA, gl::FLOAT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, g_position, 0);
        
        // - normal color buffer
        gl::GenTextures(1, &mut g_normal);
        gl::BindTexture(gl::TEXTURE_2D, g_normal);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32, dims.0 as i32, dims.1 as i32, 0, 
            gl::RGBA, gl::FLOAT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, g_normal, 0);
        
        // - diffuse color buffer
        gl::GenTextures(1, &mut g_color_diffuse);
        gl::BindTexture(gl::TEXTURE_2D, g_color_diffuse);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, dims.0 as i32, dims.1 as i32, 0, 
            gl::RGBA, gl::FLOAT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT2, gl::TEXTURE_2D, g_color_diffuse, 0);

        // Emmision color buffer
        gl::GenTextures(1, &mut g_color_emission);
        gl::BindTexture(gl::TEXTURE_2D, g_color_emission);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, dims.0 as i32, dims.1 as i32, 0, 
            gl::RGBA, gl::FLOAT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT3, gl::TEXTURE_2D, g_color_emission, 0);

        // Specular color buffer
        gl::GenTextures(1, &mut g_color_specular);
        gl::BindTexture(gl::TEXTURE_2D, g_color_specular);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, dims.0 as i32, dims.1 as i32, 0, 
            gl::RGBA, gl::FLOAT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT4, gl::TEXTURE_2D, g_color_specular, 0);
        
        // - tell OpenGL which color attachments we'll use (of this framebuffer) for rendering 
        let attachments = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2, 
            gl::COLOR_ATTACHMENT3, gl::COLOR_ATTACHMENT4];
        gl::DrawBuffers(5, &attachments[0] as *const u32);
        
        let mut rbo_depth = 0;
        gl::GenRenderbuffers(1, &mut rbo_depth);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_depth);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, dims.0 as i32, dims.1 as i32);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo_depth);
        // finally check if framebuffer is complete
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("Framebuffer not complete!");
        }
            
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
    

    let mut scene = Scene::new(view, vec![light], vec![point_light, point_light_2]);

    // Create GLSL shaders
    let program = &*shaders::BLINN_PHONG;

    let prepass_program = &*shaders::BLINN_PHONG_PREPASS;

    let lighting_program = &*shaders::BLINN_PHONG_LIGHTING;

    let point_lighting_program = &*shaders::BLINN_PHONG_POINT_LIGHTING;

    unsafe {
        gl::UseProgram(lighting_program.0); 
        let color = CString::new("color").unwrap();
        gl::BindFragDataLocation(lighting_program.0, 0, color.as_ptr());

        gl::UseProgram(0);

        gl::UseProgram(point_lighting_program.0); 
        let color = CString::new("color").unwrap();
        gl::BindFragDataLocation(point_lighting_program.0, 0, color.as_ptr());

        gl::UseProgram(0);
    }

    let angle_func = |t: f32| { (-t / 5.0) * 360.0 };
    let rotation_animation = 
        Box::new(three_d::animation::Rotation {ty: three_d::animation::RotationType::X, angle_func}) as Box<dyn three_d::animation::Animation>;  

    let mut s = Shape::from_obj(
        "media\\torus.obj", 
        ShaderType::BlinnPhong, 
        None, 
        Some(rotation_animation), 
        false, 
    ).unwrap();

    unsafe { 
        // Use shader program
        gl::UseProgram(program.0); 
        let color = CString::new("color").unwrap();
        gl::BindFragDataLocation(program.0, 0, color.as_ptr());

        s.bind_attributes(&program);
    }

    s.set_scaling(generate_scale(&[0.5; 3]));
    s.set_translation(generate_translate(None, None, Some(-2.0)));

    scene.add_shape(s, program); 

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

                    scene.draw_deferred(t, dims, &gl_window, prepass_program, lighting_program, quad_vao, 
                    g_buffer, g_position, g_normal, g_color_diffuse, g_color_emission, g_color_specular, 
                    point_lighting_program);


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