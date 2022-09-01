use std::io::Cursor;
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
}