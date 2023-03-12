pub mod directionallight;
pub mod pointlight;
pub mod spotlight;

pub const MAX_DIRECTIONAL_LIGHTS: i32 = 2;
pub const MAX_POINT_LIGHTS: i32 = 124;
pub const MAX_SPOT_LIGHTS: i32 = 2;

const AMBIENT_FACTOR: f32 = 0.03;
