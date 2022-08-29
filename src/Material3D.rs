
pub mod Material {
    use glium::texture::SrgbTexture2d;

    pub const MAX_MATERIALS: i32 = 32;

    #[derive(Copy, Clone)]
    pub struct Material<'a> {
        pub diffuse: &'a SrgbTexture2d,
        pub specular: &'a SrgbTexture2d,
        pub shininess: f32,
    }

    impl Material<'_> {
        pub fn new<'a>(diffuse: &'a SrgbTexture2d, specular: &'a SrgbTexture2d, shininess: f32) -> Material<'a> {
            return Material {
                diffuse: diffuse,
                specular: specular,
                shininess: shininess,
            }
        }
    }

}