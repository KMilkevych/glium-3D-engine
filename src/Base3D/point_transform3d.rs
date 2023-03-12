pub fn protate_X(p: [f32; 3], a: f32) -> [f32; 3] {
    return [p[0], p[1]*a.cos() - p[2]*a.sin(), p[1]*a.sin() + p[2]*a.cos()];
}

pub fn protate_Y(p: [f32; 3], a: f32) -> [f32; 3] {
    return [p[2]*a.sin() + p[0]*a.cos(), p[1], p[2]*a.cos() - p[0]*a.sin()];
}

pub fn protate_Z(p: [f32; 3], a: f32) -> [f32; 3] {
    return [p[0]*a.cos() - p[1]*a.sin(), p[0]*a.sin() + p[1]*a.cos(), p[2]];
}

pub fn ptranslate(p: [f32; 3], relative_XYZ: [f32; 3]) -> [f32; 3] {
    return [p[0] + relative_XYZ[0], p[1] + relative_XYZ[1], p[2] + relative_XYZ[2]];
}

pub fn pscale(p: [f32; 3], factor: f32) -> [f32; 3] {
    return [p[0]*factor, p[1]*factor, p[2]*factor];
}

pub fn plength(p: [f32; 3]) -> f32 {
    return (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
}

pub fn vcross(v0: [f32; 3], v1: [f32; 3]) -> [f32; 3] {
    return [
        v0[1] * v1[2] - v0[2] * v1[1],
        v0[2] * v1[0] - v0[0] * v1[2],
        v0[0] * v1[1] - v0[1] * v1[0],
    ];
}