use glium::{self, glutin::{self, event_loop::EventLoop, window::Window}, Surface, uniform, Display};
use graphics_lib::two_d;
use graphics_lib::three_d;
use graphics_lib::three_d::animation;
use graphics_lib::matrix::*;
use graphics_lib::three_d::teapot;
use image;

// Set a target for fps (don't run faster or slower than this)
const TARGET_FPS: u64 = 60;

fn main() {
    // Create event loop
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    // Initialize window and display
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    
    //demo_2d(event_loop, display);
    demo_3d_scene(event_loop, display);
}

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

fn demo_3d(event_loop: EventLoop<()>, display: Display) {
    // Create programs

    let no_shading =  glium::Program::from_source(
        &display, three_d::shaders::DEFAULT_3D_SHADER, 
        three_d::shaders::DEFAULT_3D_FRAG_SHADER, None
    ).unwrap();

    let gouraud = glium::Program::from_source(
        &display, three_d::shaders::GOURAUD_3D_SHADER, 
        three_d::shaders::GOURAUD_3D_FRAG_SHADER, None
    ).unwrap();
    
    let blinn_phong = glium::Program::from_source(
        &display, three_d::shaders::BLINN_PHONG_3D_SHADER, 
        three_d::shaders::BLINN_PHONG_3D_FRAG_SHADER, None
    ).unwrap();

    let positions  = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                        &teapot::INDICES).unwrap();


    
    let mut shape = three_d::shape::Shape::new(positions, normals, indices, 
        three_d::shaders::ShaderType::BlinnPhong, None, None, false);
    // Scale down shape
    shape.set_transform_matrix(Some(generate_scale(&[0.01; 3])), None, 
        Some(generate_translate(None, None, Some(2.0))));

    // t is our start time, delta is what we increase it by each time
    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

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
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);              

            // Draw, using the vertexs, the shaders, and the matrix
            // spin shape
            let angle = (360.0 * (t / 5.0)) * std::f32::consts::PI / 180.0;
            let rotation = generate_rotate_x(angle);

            shape.set_rotation(rotation);

            //shape.set_transform_matrix(transform);
            // light direction
            let light = [1.4, 0.4, -0.7f32];
            let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);
            shape.draw(&mut target, &light, &view, &blinn_phong);
            target.finish().unwrap();

            start_time = std::time::Instant::now();
        }
    });
}

fn demo_3d_scene(event_loop: EventLoop<()>, display: Display) {
    // Create programs

    let light = [1.4, 0.4, -0.7f32];
    let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

    let mut scene = three_d::scene::Scene::new(view, light, &display);

    let positions  = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display, 
        glium::index::PrimitiveType::TrianglesList, 
        &teapot::INDICES).unwrap();
    
    //let animation = 
    //    Box::new(animation::ConstantRotation {ty: animation::RotationType::Y, secs_per_loop: 5.0}) as Box<dyn animation::Animation>;

    let angle_func = |t: f32| { (t / 5.0) * 360.0 }; //{ ((t / 2.0).cos() + 3.0) * 90.0 };
    let rotation_animation = 
        Box::new(animation::Rotation {ty: animation::RotationType::Y, angle_func}) as Box<dyn animation::Animation>;

    let scaling_func = |t: f32| {0.01 + 0.005 * t.cos()};
    let scaling_animation = 
        Box::new(animation::Scaling {x_func: Some(scaling_func), y_func: Some(scaling_func), z_func: Some(scaling_func)}) as Box<dyn animation::Animation>;   

    let translate_func = |t: f32| {0.7 * t.sin()};
    let z_translate_func = |t: f32| {2.0 + 0.7 * t.cos()};
    let translate_animation = 
        Box::new(animation::Translation {x_func: Some(translate_func), y_func: None, z_func: Some(z_translate_func)}) as Box<dyn animation::Animation>;   

    let animation = 
        Box::new(animation::Composite {
            scaling: None, 
            rotation: Some(rotation_animation), 
            translation: Some(translate_animation)}
        ) as Box<dyn animation::Animation>;

    let mut shape = three_d::shape::Shape::new(positions, normals, indices, 
            three_d::shaders::ShaderType::BlinnPhong, None, Some(animation), false);

    
    
    // Scale down shape
    shape.set_transform_matrix(Some(generate_scale(&[0.01; 3])), None, 
        Some(generate_translate(None, None, Some(2.0))));

    let id = scene.add_shape(shape);

    // t is our start time, delta is what we increase it by each time
    let mut t: f32 = 0.0;
    let delta: f32 = 0.02;

    let mut start_time = std::time::Instant::now();

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
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);              

            scene.draw(&mut target, t);

            target.finish().unwrap();

            start_time = std::time::Instant::now();
        }
    });
    
}