use crate::Base3D::point_transform3d::*;

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: [f32; 3]
}

impl Normal {
    pub fn normal(plane: [[f32; 3]; 2]) -> Normal {
        let a = plane[0];
        let b = plane[1];
        return Normal {
            normal: vcross(a, b)
        }
    }

    pub fn rotate(&self, angle_XYZ: [f32; 3]) -> Normal {
        return Normal {
            normal: protate_Z(protate_Y(protate_X(self.normal, angle_XYZ[0]), angle_XYZ[1]), angle_XYZ[2]),
        }
    }

    /*
    Translation and Scaling of normals should not be needed,
    and will usually cause unintented behavior.
    */
}
implement_vertex!(Normal, normal);