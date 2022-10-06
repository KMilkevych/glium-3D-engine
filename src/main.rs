mod Base3D;
mod Camera3D;
mod Lights3D;
mod Uniform3D;  
mod Material3D;
mod GraphicsLoader2D;
mod Shaders;

#[macro_use]
extern crate glium;
extern crate image;

use glium::{glutin, Surface, Frame};

use crate::Base3D::General::*;
use crate::Camera3D::Camera;
use crate::Lights3D::Lights::*;
use crate::Uniform3D::Uniforms::StdUniform;
use crate::Material3D::Material::*;
use crate::GraphicsLoader2D::GraphicsLoader;


enum Action {
    Stop,
    Continue,
}



/**
 * Defining camera move and rotate speed
 */
const CAMERA_MOVE_SPEED: f32 = 0.01;
const CAMERA_ROTATE_SPEED: f32 = 0.1;

fn main() {
    // Building window and event loop
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let display = get_display(&event_loop);
    let mut is_fullscreen: bool = false;

    // Load textures
    let textures = GraphicsLoader::load_all_textures(&display);

    // Prepare program and draw parameters
    let program = glium::Program::from_source(&display, Shaders::VERTEX_SHADER, Shaders::FRAGMENT_SHADER, None).unwrap();
    let program_lights = glium::Program::from_source(&display, Shaders::VERTEX_SHADER, Shaders::FRAGMENT_SHADER_LIGHT, None).unwrap();
    let draw_parameters = get_draw_parameters();

    // Prepare fps camera
    let mut fps_camera = Camera::new([0.0, 0.0, -1.0], [0.0, 1.0, 0.0], 0.0, 90.0, CAMERA_MOVE_SPEED, CAMERA_ROTATE_SPEED);

    // Prepare static scene
    let scene = build_scene();
    let light_cube = Cube::new([0.0, 0.3, 0.0], 0.1, 0);

    // Prepare a rotating "dynamic" cube
    let mut dynamic_cube: Box<dyn Shape3D> = Box::new(Cube::new([0.0, 0.5, 0.0], 0.2, 2));

    // Describe global lighting
    let global_light: [f32; 3] = light_cube.center();

    // Run event loop
    let mut t: f32 = 0.0;
    start_loop(event_loop, move |events| {

        /*
        Update Camera 
        */

        fps_camera.update_position();
        fps_camera.update_direction();

        /*
        Update all shapes / Game objects
        */
        let mut mcb: &mut dyn Shape3D = dynamic_cube.as_mut();
        mcb = &dynamic_cube.rotate([0.0, 0.0, 0.0]);

        /*
        Combine all shapes (static scene and dynamic moving shapes) into one "package"
        to later place into single vertex buffer
        */
        
        let mut shapes: Vec<&dyn Shape3D> = Vec::new();
        shapes.push(&scene);
        shapes.push(dynamic_cube.as_ref());
        let shape = combine_shapes(&shapes);

        /*
        Create Directional and Point lights
        */
        let mut directional_lights = [    
            DirectionalLight::new([0.0, 1.0, 0.0], [0.0, 0.0, 0.0]); MAX_DIRECTIONAL_LIGHTS as usize
        ];
        //directional_lights[0] = DirectionalLight::new([-1.0, -0.6, 0.0], [1.0, 0.2, 0.2]);

        let mut point_lights = [
            PointLight::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]); MAX_POINT_LIGHTS as usize
        ];
        //point_lights[0] = PointLight::new(light_cube.center(), [1.0, 1.0, 1.0]);

        let mut spot_lights = [
            SpotLight::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, [0.0, 1.0, 0.0]); MAX_SPOT_LIGHTS as usize
        ];
        spot_lights[0] = SpotLight::new(fps_camera.get_position(), fps_camera.get_direction(), 2.0f32, [0.0, 1.0, 0.0]);

        /*
        Create materials
        */
        let mut materials = [
            Material::new(0, 0, 16.0); MAX_MATERIALS as usize
        ];
        materials[1] = Material::new(1, 1, 16.0);
        materials[2] = Material::new(2, 2, 16.0);
        materials[3] = Material::new(3, 4, 32.0);

        /*
        Beginning buffer and uniform building
        */

        // Fetch reference to display
        let mut target = display.draw(); 

        // Compute model, view and perspective matrices
        let model = get_uniform(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[0.0, 0.0, 0.0]);
        let view = fps_camera.get_view_matrix();
        let perspective = get_perspective_matrix(&target);
        
        // Create main vertex buffers
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape.get_vertices()).unwrap();
        let normal_buffer = glium::VertexBuffer::new(&display, &shape.get_normals()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Create vertex buffers for light cubes
        let lights_vertex_buffer = glium::VertexBuffer::new(&display, &light_cube.get_vertices()).unwrap();
        let lights_normal_buffer = glium::VertexBuffer::new(&display, &light_cube.get_normals()).unwrap();

        // Build uniform
        let uniform = StdUniform {
            model: model, view: view, perspective: perspective, u_light: global_light, v_view: fps_camera.get_position(),
            textures: &textures, materials: materials, num_directional_lights: 1,  directional_lights: directional_lights,
            num_point_lights: 1, point_lights: point_lights, num_spot_lights: 1, spot_lights: spot_lights
        };

        /*
        Draw everything
        */
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0); // Clear color and depth   
        target.draw((&vertex_buffer, &normal_buffer), &indices, &program, &uniform, &draw_parameters).unwrap();
        target.draw((&lights_vertex_buffer, &lights_normal_buffer), &indices, &program_lights, &uniform! {model: model, view: view, perspective: perspective}, &draw_parameters).unwrap();
        target.finish().unwrap();

        /*
        Process events
        */
        let mut action = Action::Continue;
        let mut fullscreen_toggle_pressed: bool = false;
        for event in events {
            match event {
                glutin::event::Event::DeviceEvent { event, .. } => {
                    fps_camera.process_mouse_input(&event);
                },
                glutin::event::Event::WindowEvent { event, .. } => {
                    fps_camera.process_input(&event);
                    match event {
                        glutin::event::WindowEvent::CloseRequested => action = Action::Stop,
                        glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                            glutin::event::ElementState::Pressed => match input.virtual_keycode {
                                Some(glutin::event::VirtualKeyCode::Escape) => action = Action::Stop,
                                Some(glutin::event::VirtualKeyCode::F1) => {
                                    fullscreen_toggle_pressed = true;
                                }
                                _ => (),
                            },
                            _ => (),
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        };

        if fullscreen_toggle_pressed {
            if is_fullscreen {
                display.gl_window().window().set_fullscreen(None);
            } else {
                let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
                display.gl_window().window().set_fullscreen(Some(glium::glutin::window::Fullscreen::Borderless(Some(monitor_handle))));
            }
            is_fullscreen = !is_fullscreen;
        }

        return action;
    });
}

fn build_scene() -> impl Shape3D {
    let cube1 = Cube::new([-0.5, -0.2, -0.2], 0.4, 2);
    let cube2 = Cube::new([0.1, -0.2, -0.2], 0.4, 3);
    let quad = Quad::new([-1.0, -0.2, -1.0], [[2.0, 0.0, 0.0], [0.0, 0.0, 2.0]], 1);

    let mut scene: Vec<&dyn Shape3D> = Vec::new();
    scene.push(&cube1);
    scene.push(&cube2);
    scene.push(&quad);
    
    return combine_shapes(&scene);
}

fn get_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let wb = glutin::window::WindowBuilder::new()
    .with_title("Señor Corridor")
    .with_inner_size(glium::glutin::dpi::LogicalSize::new(1280, 720));
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24).with_vsync(true);

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    display.gl_window().window().set_cursor_grab(glium::glutin::window::CursorGrabMode::Confined).unwrap();
    display.gl_window().window().set_cursor_visible(false);

    return display;
}

fn get_draw_parameters<'a>() -> glium::draw_parameters::DrawParameters<'a> {
    return glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };
}

fn get_uniform(offset: &[f32; 3], scale: &[f32; 3], rotation: &[f32; 3]) -> [[f32; 4]; 4] {
    let t = rotation[0];
    return [
        [1.0*scale[0]*t.cos(), 0.0, scale[0]*-t.sin(), 0.0],
        [0.0, 1.0*scale[1], 0.0, 0.0],
        [scale[2]*t.sin(), 0.0, 1.0*scale[2]*t.cos(), 0.0],
        [offset[0], offset[1], offset[2], 1.0f32],
    ]
}

fn get_perspective_matrix(target: &Frame) -> [[f32; 4]; 4] {
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.01;

    let f = 1.0 / (fov / 2.0).tan();
    return [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear)/(zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear)/(zfar - znear), 0.0],
    ]
}

fn start_loop<F>(event_loop: glutin::event_loop::EventLoop<()>, mut callback: F)->! where F: 'static + FnMut(&Vec<glutin::event::Event<'_, ()>>) -> Action {
    use std::time::{Instant, Duration};

    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(glutin::event::Event::NewEvents(cause)) => {
                match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } | glutin::event::StartCause::Init => {
                        true
                    },
                    _ => false
                }
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => {
                // Ignore this event.
                false
            },
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(8333334); // 120 FPS
            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            },
            Action::Stop => *control_flow = glutin::event_loop::ControlFlow::Exit
        }
    })
}