use crate::Base3D::vertex3d::Vertex;
use crate::Base3D::normal3d::Normal;
use crate::Base3D::point_transform3d::*;

pub async fn vslice_rotate(vslice: &mut [Vertex], angle_XYZ: [f32; 3]) -> () {

    for vertex in vslice.iter_mut() {
        *vertex = Vertex {
            position: protate_Z(protate_Y(protate_X(vertex .position, angle_XYZ[0]), angle_XYZ[1]), angle_XYZ[2]),
            texture: vertex .texture,
            material_id: vertex .material_id
        }
    }

}

pub async fn nslice_rotate(nslice: &mut [Normal], angle_XYZ: [f32; 3]) -> () {

    for normal in nslice.iter_mut() {
        *normal = Normal {
            normal: protate_Z(protate_Y(protate_X(normal.normal, angle_XYZ[0]), angle_XYZ[1]), angle_XYZ[2])
        }
    }

}

/*
pub async fn vslice_translate(vslice: &mut [Vertex], relative_XYZ: [f32; 3]) -> () {

    for v in vslice.iter_mut() {
        v.position = ptranslate(v.position, relative_XYZ);
    }

}

pub async fn vslice_scale(vslice: &mut [Vertex], factor: f32) -> () {

    for v in vslice.iter_mut() {
        v.position = pscale(v.position, factor);
    }

}
*/