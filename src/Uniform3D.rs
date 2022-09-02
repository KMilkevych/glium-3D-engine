#[path ="./Lights3D.rs"]
mod Lights3D;

pub mod Uniforms {
    use crate::Lights3D::Lights::*;
    use crate::Material3D::Material::*;
    use glium::texture::SrgbTexture2dArray;
    use glium::uniforms::UniformValue;

    const MAX_SAMPLERS: i32 = 32;

    pub struct StdUniform<'a> {
        pub model: [[f32; 4]; 4],
        pub view: [[f32; 4]; 4],
        pub perspective: [[f32; 4]; 4],
        pub u_light: [f32; 3],
        pub v_view: [f32; 3],
        pub textures: &'a SrgbTexture2dArray,
        pub materials: [Material; MAX_MATERIALS as usize],
        pub num_directional_lights: i32,
        pub directional_lights: [DirectionalLight; MAX_DIRECTIONAL_LIGHTS as usize],
        pub num_point_lights: i32,
        pub point_lights: [PointLight; MAX_POINT_LIGHTS as usize],
        pub num_spot_lights: i32,
        pub spot_lights: [SpotLight; MAX_SPOT_LIGHTS as usize],
    }

    impl glium::uniforms::Uniforms for StdUniform<'_> {
        fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
            f("model", UniformValue::Mat4(
                [
                [self.model[0][0], self.model[0][1], self.model[0][2], self.model[0][3]],
                [self.model[1][0], self.model[1][1], self.model[1][2], self.model[1][3]],
                [self.model[2][0], self.model[2][1], self.model[2][2], self.model[2][3]],
                [self.model[3][0], self.model[3][1], self.model[3][2], self.model[3][3]]
                ]
            ));
            f("view", UniformValue::Mat4(
                [
                [self.view[0][0], self.view[0][1], self.view[0][2], self.view[0][3]],
                [self.view[1][0], self.view[1][1], self.view[1][2], self.view[1][3]],
                [self.view[2][0], self.view[2][1], self.view[2][2], self.view[2][3]],
                [self.view[3][0], self.view[3][1], self.view[3][2], self.view[3][3]]
                ]
            ));
            f("perspective", UniformValue::Mat4(
                [
                [self.perspective[0][0], self.perspective[0][1], self.perspective[0][2], self.perspective[0][3]],
                [self.perspective[1][0], self.perspective[1][1], self.perspective[1][2], self.perspective[1][3]],
                [self.perspective[2][0], self.perspective[2][1], self.perspective[2][2], self.perspective[2][3]],
                [self.perspective[3][0], self.perspective[3][1], self.perspective[3][2], self.perspective[3][3]]
                ]
            ));
            f("u_light", UniformValue::Vec3(
                [self.u_light[0], self.u_light[1], self.u_light[2]],
            ));
            f("v_view", UniformValue::Vec3(
                [self.v_view[0], self.v_view[1], self.v_view[2]],
            ));

            f("textures", UniformValue::SrgbTexture2dArray(&self.textures, None));

            for i in 0..MAX_MATERIALS {
                f(&format!("materials[{}].diffuse",i)[..], UniformValue::SignedInt(self.materials[i as usize].diffuse));
                f(&format!("materials[{}].specular",i)[..], UniformValue::SignedInt(self.materials[i as usize].specular));
                f(&format!("materials[{}].shininess",i)[..], UniformValue::Float(self.materials[i as usize].shininess));
            }

            f("num_directional_lights", UniformValue::SignedInt(self.num_directional_lights));

            for i in 0..MAX_DIRECTIONAL_LIGHTS {
                f(&format!("directional_lights[{}].direction",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].direction));
                f(&format!("directional_lights[{}].ambient_color",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].ambient_color));
                f(&format!("directional_lights[{}].diffuse_color",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].diffuse_color));
                f(&format!("directional_lights[{}].specular_color",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].specular_color));
            }

            f("num_point_lights", UniformValue::SignedInt(self.num_point_lights));

            for i in 0..MAX_POINT_LIGHTS {
                f(&format!("point_lights[{}].position",i)[..], UniformValue::Vec3(self.point_lights[i as usize].position));
                f(&format!("point_lights[{}].constant",i)[..], UniformValue::Float(self.point_lights[i as usize].constant));
                f(&format!("point_lights[{}].linear",i)[..], UniformValue::Float(self.point_lights[i as usize].linear));
                f(&format!("point_lights[{}].quadratic",i)[..], UniformValue::Float(self.point_lights[i as usize].quadratic));
                f(&format!("point_lights[{}].ambient_color",i)[..], UniformValue::Vec3(self.point_lights[i as usize].ambient_color));
                f(&format!("point_lights[{}].diffuse_color",i)[..], UniformValue::Vec3(self.point_lights[i as usize].diffuse_color));
                f(&format!("point_lights[{}].specular_color",i)[..], UniformValue::Vec3(self.point_lights[i as usize].specular_color));
            }

            f("num_spot_lights", UniformValue::SignedInt(self.num_spot_lights));

            for i in 0..MAX_SPOT_LIGHTS {
                f(&format!("spot_lights[{}].position",i)[..], UniformValue::Vec3(self.spot_lights[i as usize].position));
                f(&format!("spot_lights[{}].direction",i)[..], UniformValue::Vec3(self.spot_lights[i as usize].direction));
                f(&format!("spot_lights[{}].cutoff",i)[..], UniformValue::Float(self.spot_lights[i as usize].cutoff));
                f(&format!("spot_lights[{}].outer_cutoff",i)[..], UniformValue::Float(self.spot_lights[i as usize].outer_cutoff));
                f(&format!("spot_lights[{}].constant",i)[..], UniformValue::Float(self.spot_lights[i as usize].constant));
                f(&format!("spot_lights[{}].linear",i)[..], UniformValue::Float(self.spot_lights[i as usize].linear));
                f(&format!("spot_lights[{}].quadratic",i)[..], UniformValue::Float(self.spot_lights[i as usize].quadratic));
                f(&format!("spot_lights[{}].ambient_color",i)[..], UniformValue::Vec3(self.spot_lights[i as usize].ambient_color));
                f(&format!("spot_lights[{}].diffuse_color",i)[..], UniformValue::Vec3(self.spot_lights[i as usize].diffuse_color));
                f(&format!("spot_lights[{}].specular_color",i)[..], UniformValue::Vec3(self.spot_lights[i as usize].specular_color));
            }
        }
    }

}