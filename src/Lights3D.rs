

pub mod Lights {

    pub const MAX_DIRECTIONAL_LIGHTS: u32 = 10;
    pub const MAX_POINT_LIGHTS: u32 = 100;
    pub const MAX_SPOT_LIGHTS: u32 = 100;

    #[derive(Copy, Clone)]
    pub struct DirectionalLight {
        pub direction: [f32; 3],

        pub ambient_color: [f32; 3],
        pub diffuse_color: [f32; 3],
        pub specular_color: [f32; 3],
    }

    impl DirectionalLight {
        pub fn new(direction: [f32; 3], color: [f32; 3]) -> DirectionalLight{
            return DirectionalLight {
                direction: direction,
                ambient_color: [color[0]*0.1, color[1]*0.1, color[2]*0.1],
                diffuse_color: color,
                specular_color: color,
            }
        }

        pub fn new_colors(direction: [f32; 3], ambient_color: [f32; 3], diffuse_color: [f32; 3], specular_color: [f32; 3]) -> DirectionalLight{
            return DirectionalLight {
                direction: direction,
                ambient_color: ambient_color,
                diffuse_color: diffuse_color,
                specular_color: specular_color,
            }
        }
    }
}