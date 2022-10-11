pub mod General {

    fn protate_X(p: [f32; 3], a: f32) -> [f32; 3] {
        return [p[0], p[1]*a.cos() - p[2]*a.sin(), p[1]*a.sin() + p[2]*a.cos()];
    }

    fn protate_Y(p: [f32; 3], a: f32) -> [f32; 3] {
        return [p[2]*a.sin() + p[0]*a.cos(), p[1], p[2]*a.cos() - p[0]*a.sin()];
    }

    fn protate_Z(p: [f32; 3], a: f32) -> [f32; 3] {
        return [p[0]*a.cos() - p[1]*a.sin(), p[0]*a.sin() + p[1]*a.cos(), p[2]];
    }

    fn ptranslate(p: [f32; 3], relative_XYZ: [f32; 3]) -> [f32; 3] {
        return [p[0] + relative_XYZ[0], p[1] + relative_XYZ[1], p[2] + relative_XYZ[2]];
    }

    fn pscale(p: [f32; 3], factor: f32) -> [f32; 3] {
        return [p[0]*factor, p[1]*factor, p[2]*factor];
    }

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

    #[derive(Copy, Clone)]
    pub struct Normal {
        pub normal: [f32; 3]
    }

    impl Normal {
        pub fn normal(plane: [[f32; 3]; 2]) -> Normal {
            let a = plane[0];
            let b = plane[1];
            return Normal {
                normal: [
                    a[1] * b[2] - a[2] * b[1],
                    a[2] * b[0] - a[0] * b[2],
                    a[0] * b[1] - a[1] * b[0],
                    ],
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

    /*
     * General shapes 
     */
    pub trait Shape3D {
        fn get_vertices(&mut self) -> &mut Vec<Vertex>;
        fn get_normals(&mut self) -> &mut Vec<Normal>;

        fn rotate(&mut self, angle_XYZ: [f32; 3]) -> AShape {

            let mut vertices: Vec<Vertex> = Vec::new();
            let mut normals: Vec<Normal> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                vertices.push(vertex.rotate(angle_XYZ));
            }
    
            for normal in self.get_normals().iter() {
                normals.push(normal.rotate(angle_XYZ));
            }
    
            return AShape {
                vertices: vertices,
                normals: normals
            };
        }

        fn rotate_O(&mut self, angle_XYZ: [f32; 3]) -> AShape {
            let origin: [f32; 3] = self.centroid();

            let mut vertices: Vec<Vertex> = Vec::new();
            let mut normals: Vec<Normal> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                let v: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
                let v: Vertex = v.rotate(angle_XYZ);
                let v: Vertex = v.translate(origin);
                vertices.push(v);
            }
    
            for normal in self.get_normals().iter() {
                // Don't translate normals
                let n: Normal = normal.rotate(angle_XYZ);
                normals.push(n);
            }
    
            return AShape {
                vertices: vertices,
                normals: normals
            };
        }

        fn translate(&mut self, relative_XYZ: [f32; 3]) -> AShape {
            let mut vertices: Vec<Vertex> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                let v: Vertex = vertex.translate(relative_XYZ);
                vertices.push(v);
            }
    
            return AShape {
                vertices: vertices,
                normals: self.get_normals().to_vec()
            };
        }

        fn scale_O(&mut self, factor: f32) -> AShape {
            let origin: [f32; 3] = self.centroid();
            let mut vertices: Vec<Vertex> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                let v: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
                let v: Vertex = v.scale(factor);
                let v: Vertex = v.translate(origin);
                vertices.push(v);
            }

            return AShape {
                vertices: vertices.to_vec(),
                normals: self.get_normals().to_vec()
            };
        }

        fn centroid(&mut self) -> [f32; 3] {
            let mut n: i32 = 0;
            let mut sum: [f32; 3] = [0.0; 3];
            for vertex in self.get_vertices() {
                for i in 0..3 {
                    sum[i] += vertex.position[i];
                }
                n += 1;
            }
            return [sum[0]/(n as f32), sum[1]/(n as f32), sum[2]/(n as f32)];
        }

    }

    #[derive(Clone)]
    pub struct AShape {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Shape3D for AShape {

        /*
        fn unm_get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn unm_get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }
        */

        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub struct Quad {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Quad {
        pub fn new(bottom_left: [f32; 3], plane: [[f32; 3]; 2], material_id: i32) -> AShape {
            let bl = bottom_left;
            let rv = plane[0];
            let lv = plane[1];
            return AShape {
                vertices: vec![
                    Vertex {position: [bl[0],                   bl[1],                  bl[2]],                     texture: [0.0, 0.0], material_id},
                    Vertex {position: [bl[0] + rv[0],           bl[1] + rv[1],          bl[2] + rv[2]],             texture: [1.0, 0.0], material_id},
                    Vertex {position: [bl[0] + rv[0] + lv[0],   bl[1] + rv[1] + lv[1],  bl[2] + rv[2] + lv[2]],     texture: [1.0, 1.0], material_id},
                    
                    Vertex {position: [bl[0],                   bl[1],                  bl[2]],                     texture: [0.0, 0.0], material_id},
                    Vertex {position: [bl[0] + rv[0] + lv[0],   bl[1] + rv[1] + lv[1],  bl[2] + rv[2] + lv[2]],     texture: [1.0, 1.0], material_id},
                    Vertex {position: [bl[0] + lv[0],           bl[1] + lv[1],          bl[2] + lv[2]],             texture: [0.0, 1.0], material_id},                    
                ],
                normals: vec![Normal::normal(plane); 6],
            }
        }
    }

    impl Shape3D for Quad {
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub struct Cube {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Cube {
        pub fn new(bottom_front_left: [f32; 3], side_length: f32, material_id: i32) -> AShape {

            let sl = side_length;
            let bfl = bottom_front_left;

            let mut top: AShape =       Quad::new([bfl[0],          bfl[1] + sl,    bfl[2]],        [[sl, 0.0, 0.0], [0.0, 0.0, sl]], material_id); // Top
            let mut bottom: AShape =    Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],        [[sl, 0.0, 0.0], [0.0, 0.0, -sl]], material_id); // Bottom
            let mut front: AShape =     Quad::new([bfl[0],          bfl[1],         bfl[2]],        [[sl, 0.0, 0.0], [0.0, sl, 0.0]], material_id); // Front
            let mut rear: AShape =      Quad::new([bfl[0] + sl,     bfl[1],         bfl[2] + sl],   [[-sl, 0.0, 0.0], [0.0, sl, 0.0]], material_id); // Rear
            let mut left: AShape =      Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],   [[0.0, 0.0, -sl], [0.0, sl, 0.0]], material_id); // Left
            let mut right: AShape =     Quad::new([bfl[0] + sl,     bfl[1],         bfl[2]],        [[0.0, 0.0, sl], [0.0, sl, 0.0]], material_id); // Right

            let quads: Vec<&mut AShape> = vec! [&mut top, &mut bottom, &mut front, &mut rear, &mut left, &mut right];
            let mut cube = combine_shapes(quads);

            return AShape {
                vertices: cube.get_vertices().to_owned(),
                normals: cube.get_normals().to_owned(),
            }
        }
    }

    impl Shape3D for Cube {
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub fn combine_shapes(mut shapes: Vec<&mut AShape>) -> AShape {

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();

        for i in 0..shapes.len() {
            let mut shape: &mut AShape = shapes[i];
            
            for j in 0..shape.get_vertices().len() {
                vertices.push((shape.get_vertices()[j]).clone());
                normals.push((shape.get_normals()[j]).clone());
            }
            
        }

        return AShape {
            vertices: vertices,
            normals: normals,
        }
    }

}