use crate::Lights3D::AMBIENT_FACTOR;

#[derive(Copy, Clone)]
pub struct PointLight {
    pub position: [f32; 3],

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
}

impl PointLight {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> PointLight {
        // Create standard point light with distance coverage 32
        return PointLight {
            position: position,
            constant: 1.0,
            linear: 0.14,
            quadratic: 0.07,
            ambient_color: [color[0]*AMBIENT_FACTOR, color[1]*AMBIENT_FACTOR, color[2]*AMBIENT_FACTOR],
            diffuse_color: color,
            specular_color: color,
        }
    }

    pub fn new_with_parameters(position: [f32; 3], color: [f32; 3], linear: f32, quadratic: f32) -> PointLight {
        // Create standard point light with user specified parameters
        return PointLight {
            position: position,
            constant: 1.0,
            linear: linear,
            quadratic: quadratic,
            ambient_color: [color[0]*AMBIENT_FACTOR, color[1]*AMBIENT_FACTOR, color[2]*AMBIENT_FACTOR],
            diffuse_color: color,
            specular_color: color,
        }
    }
}