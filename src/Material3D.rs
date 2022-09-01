
pub mod Material {
    //use glium::texture::SrgbTexture2d;

    pub const MAX_MATERIALS: i32 = 32;

    #[derive(Copy, Clone)]
    pub struct Material {
        pub diffuse: i32, // diffuse texture id
        pub specular: i32, // specular texture id
        pub shininess: f32,
    }

    impl Material {
        pub fn new(diffuse: i32, specular: i32, shininess: f32) -> Material {
            return Material {
                diffuse: diffuse,
                specular: specular,
                shininess: shininess,
            }
        }
    }

}