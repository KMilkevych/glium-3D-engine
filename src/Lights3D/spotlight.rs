use crate::Lights3D::AMBIENT_FACTOR;

#[derive(Copy, Clone)]
pub struct SpotLight {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub cutoff: f32,
    pub outer_cutoff: f32,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
}

impl SpotLight {
    pub fn new(position: [f32; 3], direction: [f32; 3], cutoff: f32, color: [f32; 3]) -> SpotLight{
        return SpotLight { 
            position: position,
            direction: direction,
            cutoff: cutoff.to_radians().cos(),
            outer_cutoff: (cutoff + 24.0f32).to_radians().cos(),
            constant: 1.0,
            linear: 0.14,
            quadratic: 0.07,
            ambient_color: [color[0]*AMBIENT_FACTOR, color[1]*AMBIENT_FACTOR, color[2]*AMBIENT_FACTOR],
            diffuse_color: color,
            specular_color: color,
        }
    }
}