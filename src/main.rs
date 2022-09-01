
mod Base3D;
mod Camera3D;
mod Lights3D;
mod Uniform3D;  
mod Material3D;

#[macro_use]
extern crate glium;
extern crate image;

use glium::texture::{SrgbTexture2d, SrgbTexture2dArray, Texture2dDataSource, PixelValue};
use glium::texture::srgb_texture2d_array::SrgbTexture2dArrayMipmap;
use glium::{glutin, Surface, Frame};
use crate::Base3D::General::*;
use crate::Camera3D::Camera;
use crate::Lights3D::Lights::*;
use crate::Uniform3D::Uniforms::StdUniform;
use crate::Material3D::Material::*;

use std::io::Cursor;
use std::fmt;

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
    in int material_id;

    out vec3 v_normal;
    out vec3 v_position;
    out vec2 v_texture;
    flat out int i_material;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        v_texture = texture;
        i_material = material_id;

        mat4 modelview = view * model;
        
        gl_Position = perspective * modelview * vec4(position, 1.0);

        v_position = vec3(model * vec4(position, 1.0));
        v_normal = transpose(inverse(mat3(model))) * normal;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 150

    struct Material {
        int diffuse;
        int specular;
        float shininess;
    };

    struct DirectionalLight {
        vec3 direction;
        vec3 ambient_color;
        vec3 diffuse_color;
        vec3 specular_color;
    };

    struct PointLight {
        vec3 position;
        float constant;
        float linear;
        float quadratic;
        vec3 ambient_color;
        vec3 diffuse_color;
        vec3 specular_color;
    };

    in vec3 v_normal;
    in vec3 v_position;
    in vec2 v_texture;
    flat in int i_material;

    out vec4 color;

    uniform vec3 u_light;
    uniform vec3 v_view;

    uniform sampler2DArray textures;
    uniform Material materials[32];

    uniform int num_directional_lights;
    uniform DirectionalLight directional_lights[4];

    uniform int num_point_lights;
    uniform PointLight point_lights[128];

    vec3 calc_dir_light(DirectionalLight light, vec3 normal, vec3 view_dir) {
        vec3 light_dir = normalize(-light.direction);

        float diff = max(dot(normal, -light_dir), 0.0);
        
        vec3 reflect_dir = reflect(-light_dir, normal);
        float spec = pow(max(dot(view_dir, reflect_dir), 0.0), materials[i_material].shininess);

        vec3 ambient = light.ambient_color * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 diffuse = light.diffuse_color * diff * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 specular = light.specular_color * spec * vec3(texture(textures, vec3(v_texture, materials[i_material].specular)));

        return (ambient + diffuse + specular);
    }

    vec3 calc_point_light(PointLight light, vec3 normal, vec3 position, vec3 view_dir) {
        float distance = length(light.position - position);
        float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

        vec3 light_dir = -normalize(light.position - position);

        float diff = max(dot(normal, light_dir), 0.0);
        
        vec3 reflect_dir = reflect(light_dir, normal);
        float spec = pow(max(dot(view_dir, reflect_dir), 0.0), materials[i_material].shininess);

        vec3 ambient = light.ambient_color * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 diffuse = light.diffuse_color * diff * vec3(texture(textures, vec3(v_texture, materials[i_material].diffuse)));
        vec3 specular = light.specular_color * spec * vec3(texture(textures, vec3(v_texture, materials[i_material].specular)));

        return (ambient + diffuse + specular)*attenuation;
    }

    void main() {
        vec3 res_color = vec3(0.0, 0.0, 0.0);

        vec3 norm = normalize(v_normal);
        vec3 view_dir = normalize(v_view - v_position);

        for (int i = 0; i < num_directional_lights; i++) {
            res_color += calc_dir_light(directional_lights[i], norm, view_dir);
        }

        for (int i = 0; i < num_point_lights; i++) {
            res_color += calc_point_light(point_lights[i], norm, v_position, view_dir);
        }

        color = vec4(res_color, 1.0);
       
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
    let mut is_fullscreen: bool = false;

    // Load textures
    /*
    let texture_wall = load_texture(&display, include_bytes!("textures/tex_wall.jpg"), image::ImageFormat::Jpeg);
    let texture_box = load_texture(&display, include_bytes!("textures/tex_box.jpg"), image::ImageFormat::Jpeg);
    let texture_metal_box = load_texture(&display, include_bytes!("textures/tex_metal_box.png"), image::ImageFormat::Png);
    let specular_metal_box = load_texture(&display, include_bytes!("textures/spec_metal_box.png"), image::ImageFormat::Png);
    */

    let texture_err = load_image_from_color([1.0, 0.0, 1.0]);

    let texture_wall = load_image(include_bytes!("textures/tex_wall.jpg"), image::ImageFormat::Jpeg);
    let texture_box = load_image(include_bytes!("textures/tex_box.jpg"), image::ImageFormat::Jpeg);
    let texture_metal_box = load_image(include_bytes!("textures/tex_metal_box.png"), image::ImageFormat::Png);
    let specular_metal_box = load_image(include_bytes!("textures/spec_metal_box.png"), image::ImageFormat::Png);

    let textures: SrgbTexture2dArray = SrgbTexture2dArray::new(&display, vec! [texture_err, texture_wall, texture_box, texture_metal_box, specular_metal_box]).unwrap();

    // Prepare program and draw parameters
    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let program_lights = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_LIGHT, None).unwrap();
    let draw_parameters = get_draw_parameters();

    // Prepare fps camera
    let mut fps_camera = Camera::new([0.0, 0.0, -1.0], [0.0, 1.0, 0.0], 0.0, 90.0, CAMERA_MOVE_SPEED, CAMERA_ROTATE_SPEED);

    // Prepare static scene
    let scene = build_scene();
    let light_cube = Cube::new([0.0, 0.3, 0.0], 0.1, 0);

    // Describe global lighting
    let global_light: [f32; 3] = light_cube.center();

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
        Create lights
        */
        let mut directional_lights = [    
            DirectionalLight::new([0.0, 1.0, 0.0], [0.0, 0.0, 0.0]); MAX_DIRECTIONAL_LIGHTS as usize
        ];
        //directional_lights[0] = DirectionalLight::new([1.0, -0.6, 0.2], [1.0, 1.0, 1.0]);

        let mut point_lights = [
            PointLight::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]); MAX_POINT_LIGHTS as usize
        ];
        point_lights[0] = PointLight::new(light_cube.center(), [1.0, 1.0, 0.4]);

        /*
        Create materials
        */
        let mut materials = [
            Material::new(0, 0, 0.0); MAX_MATERIALS as usize
        ];
        materials[1] = Material::new(1, 1, 16.0);
        materials[2] = Material::new(2, 2, 16.0);
        materials[3] = Material::new(3, 4, 32.0);

        let mut target = display.draw(); // Fetch the display

        let model = get_uniform(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[0.0, 0.0, 0.0]);
        let view = fps_camera.get_view_matrix();
        let perspective = get_perspective_matrix(&target);
        
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape.get_vertices()).unwrap();
        let normal_buffer = glium::VertexBuffer::new(&display, &shape.get_normals()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let lights_vertex_buffer = glium::VertexBuffer::new(&display, &light_cube.get_vertices()).unwrap();
        let lights_normal_buffer = glium::VertexBuffer::new(&display, &light_cube.get_normals()).unwrap();

        // Build uniforms
        let uniform = StdUniform {
            model: model, view: view, perspective: perspective, u_light: global_light, v_view: fps_camera.get_position(),
            textures: &textures, materials: materials, num_directional_lights: 1,  directional_lights: directional_lights,
            num_point_lights: 1, point_lights: point_lights
        };

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



fn load_image(bytes: &'static [u8], format: image::ImageFormat) -> glium::texture::RawImage2d<u8> {
    let image = image::load(Cursor::new(bytes), format).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    return image;
}

fn load_image_from_color(color: [f32; 3]) -> glium::texture::RawImage2d<'static, u8> {
    let (dim_x, dim_y) = (512, 512);
    let mut image_buffer = image::ImageBuffer::<image::Rgba<u8>, _>::new(dim_x, dim_y);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let r: u8 = (color[0]*255.0).floor() as u8;
        let g: u8 = (color[1]*255.0).floor() as u8;
        let b: u8 = (color[2]*255.0).floor() as u8;
        *pixel = image::Rgba([r, g, b, 255 as u8]);
    }
    return glium::texture::RawImage2d::from_raw_rgba_reversed(&image_buffer.into_raw(), (dim_x, dim_y));
}

fn load_texture(display: &glium::Display, bytes: &'static [u8], format: image::ImageFormat) -> SrgbTexture2d {
    return glium::texture::SrgbTexture2d::new(display, load_image(bytes, format)).unwrap();
}

fn load_texture_from_color(display: &glium::Display, color: [f32; 3]) -> SrgbTexture2d {
    return glium::texture::SrgbTexture2d::new(display, load_image_from_color(color)).unwrap();
}

fn create_texture_array(display: &glium::Display) -> SrgbTexture2dArray {
    let image1 = load_image(include_bytes! ("textures/tex_wall.jpg"), image::ImageFormat::Jpeg);
    let image2 = load_image(include_bytes! ("textures/tex_wall.jpg"), image::ImageFormat::Jpeg);
    
    return SrgbTexture2dArray::new(display, vec! [image1, image2]).unwrap();
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
    let znear = 0.01; // Reduced from 0.1 to 0.01 to prevent visually clipping through walls

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