#[path ="./Lights3D.rs"]
mod Lights3D;

pub mod Uniforms {
    use crate::Lights3D::Lights::*;
    use glium::uniforms::UniformValue;


    pub struct StdUniform {
        pub model: [[f32; 4]; 4],
        pub view: [[f32; 4]; 4],
        pub perspective: [[f32; 4]; 4],
        pub u_light: [f32; 3],
        pub v_view: [f32; 3],
        pub directional_lights: [DirectionalLight; 10],
    }

    impl glium::uniforms::Uniforms for StdUniform {
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

            for i in 0..10 {
                f(&format!("directional_lights[{}].direction",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].direction));
                f(&format!("directional_lights[{}].ambient_color",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].ambient_color));
                f(&format!("directional_lights[{}].diffuse_color",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].diffuse_color));
                f(&format!("directional_lights[{}].specular_color",i)[..], UniformValue::Vec3(self.directional_lights[i as usize].specular_color));
            }
        }
    }

}