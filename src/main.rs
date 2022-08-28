
mod Base3D;
mod Camera3D;

#[macro_use]
extern crate glium;

use glium::{glutin, Surface, Frame};
use crate::Base3D::General::*;
use crate::Camera3D::Camera;

enum Action {
    Stop,
    Continue,
}

/*
* Defining vertex and fragment shaders
*/

const VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    in vec2 texture;

    out vec3 v_normal;
    out vec3 v_position;
    out vec2 v_texture;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        v_texture = texture;

        mat4 modelview = view * model;
        
        gl_Position = perspective * modelview * vec4(position, 1.0);

        v_position = vec3(model * vec4(position, 1.0));
        v_normal = transpose(inverse(mat3(model))) * normal;
        
        //v_position = gl_Position.xyz / gl_Position.w;
        //v_normal = transpose(inverse(mat3(modelview))) * normal;
        
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 150

    in vec3 v_normal;
    in vec3 v_position;
    in vec2 v_texture;

    out vec4 color;

    uniform vec3 u_light;
    uniform vec3 v_view;

    const vec3 ambient_color = vec3(0.1, 0.1, 0.1);
    const vec3 diffuse_color = vec3(0.8, 0.8, 0.8);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);

    const float ambient_intensity = 0.1;
    const float specular_intensity = 1.0;

    void main() {
        vec3 tex_color = vec3(v_texture, 1.0);

        vec3 norm = normalize(v_normal);
        vec3 light_dir = normalize(-(u_light - v_position));

        float diffuse = max(dot(norm, light_dir), 0.0);
        vec3 reflected_light = reflect(light_dir, norm);

        vec3 view_dir = normalize(v_view - v_position);
        vec3 reflect_dir = reflect(light_dir, norm);
        float specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32);

        vec3 resulting_color = ((ambient_color * ambient_intensity) + (diffuse_color * diffuse) + (specular * specular_intensity * specular_color))*tex_color;
        color = vec4(resulting_color, 1.0);

        //vec3 camera_dir = normalize(-v_position);
        //vec3 half_direction = normalize(normalize(u_light) + camera_dir);

        //float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
        
        //vec3 tex_color = vec3(v_texture, 1.0);
        //color = vec4(ambient_color + diffuse * tex_color + specular * specular_color, 1.0);

    }
"#;

const FRAGMENT_SHADER_LIGHT: &str = r#"
    #version 150

    in vec3 v_normal;
    in vec3 v_position;
    in vec2 v_texture;

    out vec4 color;

    const vec3 v_color = vec3(1.0, 1.0, 1.0);

    void main() {
        color = vec4(v_color, 1.0);
    }
"#;

/**
 * Defining camera move and rotate speed
 */
const CAMERA_MOVE_SPEED: f32 = 0.01;
const CAMERA_ROTATE_SPEED: f32 = 0.1;

fn main() {
    
    // Building window and event loop
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let display = get_display(&event_loop);

    // Prepare program and draw parameters
    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let program_lights = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_LIGHT, None).unwrap();
    let draw_parameters = get_draw_parameters();

    // Prepare static scene
    let scene = build_scene();
    let light_cube = Cube::new([-0.4, 0.6, -0.2], 0.1);

    // Describe global lighting
    let global_light: [f32; 3] = light_cube.center();

    // Prepare fps camera
    let mut fps_camera = Camera::new([0.0, 0.0, -1.0], [0.0, 1.0, 0.0], 0.0, 90.0, CAMERA_MOVE_SPEED, CAMERA_ROTATE_SPEED);

    // Run event loop
    let mut t: f32 = 0.0;
    start_loop(event_loop, move |events| {

        fps_camera.update_position();
        fps_camera.update_direction();

        /*
        Update all objects/shapes
        */
        t += 0.001;
        if t > 6.283 {
            t = 0.0;
        }

        /*
        Combine all shapes (static scene and dynamic moving shapes) into one "package"
        */
        
        let mut shapes: Vec<&dyn Shape3D> = Vec::new();
        shapes.push(&scene);

        let shape = combine_shapes(&shapes);

        /*
            Drawing phase (draw everything)
        */

        let mut target = display.draw(); // Fetch the display

        let uniform_matrix = get_uniform(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[0.0, 0.0, 0.0]);
        let view = get_view_matrix(&fps_camera.get_position(), &fps_camera.get_direction(), &fps_camera.get_orientation());
        let perspective = get_perspective_matrix(&target);
        
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape.get_vertices()).unwrap();
        let normal_buffer = glium::VertexBuffer::new(&display, &shape.get_normals()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let lights_vertex_buffer = glium::VertexBuffer::new(&display, &light_cube.get_vertices()).unwrap();
        let lights_normal_buffer = glium::VertexBuffer::new(&display, &light_cube.get_normals()).unwrap();

        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0); // Clear color and depth   
        target.draw((&vertex_buffer, &normal_buffer), &indices, &program, &uniform! {
            model: uniform_matrix, view: view, perspective: perspective,
            u_light: global_light, v_view: fps_camera.get_position()}, &draw_parameters).unwrap();
        target.draw((&lights_vertex_buffer, &lights_normal_buffer), &indices, &program_lights, &uniform! {model: uniform_matrix, view: view, perspective: perspective}, &draw_parameters).unwrap();
        target.finish().unwrap();

        /*
            Process events
        */
        let mut action = Action::Continue;
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
        return action;
    });
}

fn build_scene() -> impl Shape3D {
    let cube1 = Cube::new([-0.5, -0.2, -0.2], 0.4);
    let cube2 = Cube::new([0.1, -0.2, -0.2], 0.4);
    let quad = Quad::new([-1.0, -0.2, -1.0], [[2.0, 0.0, 0.0], [0.0, 0.0, 2.0]]);

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
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();
    return [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear)/(zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear)/(zfar - znear), 0.0],
    ]
}

fn get_view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    return [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
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
            next_frame_time = Instant::now() + Duration::from_nanos(8333334);
            // TODO: Add back the old accumulator loop in some way

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