use crate::Base3D::point_transform3d::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture: [f32; 2],
    pub material_id: i32,
}

impl Vertex {

    pub fn rotate(&self, angle_XYZ: [f32; 3]) -> Vertex {
        return Vertex {
            position: protate_Z(protate_Y(protate_X(self.position, angle_XYZ[0]), angle_XYZ[1]), angle_XYZ[2]),
            texture: self.texture,
            material_id: self.material_id
        }
    }

    pub fn translate(&self, relative_XYZ: [f32; 3]) -> Vertex {
        return Vertex {
            position: ptranslate(self.position, relative_XYZ),
            texture: self.texture,
            material_id: self.material_id
        }
    }

    pub fn scale(&self, factor: f32) -> Vertex {
        return Vertex {
            position: pscale(self.position, factor),
            texture: self.texture,
            material_id: self.material_id
        }
    }

}
implement_vertex!(Vertex, position, texture, material_id);