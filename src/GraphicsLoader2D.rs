use std::io::{Cursor, Read};
use std::fs;
use std::path::PathBuf;
use glium;

pub struct GraphicsLoader {}
impl GraphicsLoader {
    pub fn load_image(bytes: &'static [u8], format: image::ImageFormat) -> glium::texture::RawImage2d<u8> {
        let image = image::load(Cursor::new(bytes), format).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        return image;
    }
    
    pub fn load_image_from_color(color: [f32; 3]) -> glium::texture::RawImage2d<'static, u8> {
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
    
    pub fn load_texture(display: &glium::Display, bytes: &'static [u8], format: image::ImageFormat) -> glium::texture::SrgbTexture2d {
        return glium::texture::SrgbTexture2d::new(display, GraphicsLoader::load_image(bytes, format)).unwrap();
    }
    
    pub fn load_texture_from_color(display: &glium::Display, color: [f32; 3]) -> glium::texture::SrgbTexture2d {
        return glium::texture::SrgbTexture2d::new(display, GraphicsLoader::load_image_from_color(color)).unwrap();
    }
    
    pub fn create_texture_array(display: &glium::Display, images: Vec<glium::texture::RawImage2d<u8>>) -> glium::texture::SrgbTexture2dArray {
        return glium::texture::SrgbTexture2dArray::new(display, images).unwrap();
    }

    pub fn load_all_textures(display: &glium::Display) -> glium::texture::SrgbTexture2dArray {
        
        let mut images: Vec<glium::texture::RawImage2d<u8>> = Vec::new();
        images.push(GraphicsLoader::load_image_from_color([1.0, 0.0, 1.0]));
        images.push(GraphicsLoader::load_image(include_bytes!("textures/t_001.jpg"), image::ImageFormat::Jpeg));
        images.push(GraphicsLoader::load_image(include_bytes!("textures/t_002.jpg"), image::ImageFormat::Jpeg));
        images.push(GraphicsLoader::load_image(include_bytes!("textures/t_003.png"), image::ImageFormat::Png));
        images.push(GraphicsLoader::load_image(include_bytes!("textures/t_004.png"), image::ImageFormat::Png));
        return GraphicsLoader::create_texture_array(display, images);

        /*
        let mut resource_dir = std::env::current_exe().unwrap();
        resource_dir.pop();
        resource_dir.push(directory);

        let base_dir = option_env!("CARGO_MANIFEST_DIR").map_or_else(|| {
            let exe_path = std::env::current_exe().expect("Failed to get exe path");
            exe_path.parent().expect("Failed to get exe dir").to_path_buf()
        }, |crate_dir| {
            let mut pb = PathBuf::new();
            pb.push(crate_dir);
            pb.push("src");
            pb
        });

        let mut images: Vec<glium::texture::RawImage2d<u8>> = Vec::new();

        let paths = fs::read_dir(base_dir.join(directory)).unwrap();
        for path in paths {
            
            let mut image_format: image::ImageFormat;
            let path = path.unwrap().path();
            match path.extension() {
                Some(os_str) => {
                    match os_str.to_str() {
                        Some("jpg") => {image_format = image::ImageFormat::Jpeg},
                        Some("png") => {image_format = image::ImageFormat::Png},
                        _ => continue,
                    }
                },
                None => continue,
            }

            let f = fs::File::open(path).unwrap();
            let mut reader = std::io::BufReader::new(f);
            let mut bytes: Vec<u8> = Vec::new();
            reader.read_to_end(&mut bytes);

            images.push(GraphicsLoader::load_image_direct(bytes, image_format));
        }

        return GraphicsLoader::create_texture_array(display, images);
        */

    }
}