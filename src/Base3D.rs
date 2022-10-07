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

    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 3],
        pub texture: [f32; 2],
        pub material_id: i32,
    }
    
    impl Vertex {
        /*
        pub fn rotate_X(&self, angle: f32) -> Vertex {
            return Vertex {
                position: protate_X(self.position, angle),
                texture: self.texture,
                material_id: self.material_id
            }
        }

        pub fn rotate_Y(&self, angle: f32) -> Vertex {
            return Vertex {
                position: protate_Y(self.position, angle),
                texture: self.texture,
                material_id: self.material_id
            }
        }

        pub fn rotate_Z(&self, angle: f32) -> Vertex {
            return Vertex {
                position: protate_Z(self.position, angle),
                texture: self.texture,
                material_id: self.material_id
            }
        }
        */

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

        pub fn translate(&self, relative_XYZ: [f32; 3]) -> Normal {
            return Normal {
                normal: ptranslate(self.normal, relative_XYZ),
            }
        }
    }
    implement_vertex!(Normal, normal);

    /*
     * General shapes 
     */

    pub trait Shape3D {
        fn get_vertices(&self) -> Vec<Vertex>;
        fn get_normals(&self) -> Vec<Normal>;

        fn rotate(&self, angle_XYZ: [f32; 3]) -> Box<dyn Shape3D> {

            let mut vertices: Vec<Vertex> = Vec::new();
            let mut normals: Vec<Normal> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                vertices.push(vertex.rotate(angle_XYZ));
            }
    
            for normal in self.get_normals().iter() {
                normals.push(normal.rotate(angle_XYZ));
            }
    
            return Box::new(AShape {
                vertices: vertices,
                normals: normals
            });
        }

        fn rotate_O(&self, angle_XYZ: [f32; 3]) -> Box<dyn Shape3D> {
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
                //let n: Normal = normal.translate([-origin[0], -origin[1], -origin[2]]);
                //let n: Normal = n.rotate(angle_XYZ);
                //let n: Normal = n.translate(origin);
                let n: Normal = normal.rotate(angle_XYZ);
                normals.push(n);
            }
    
            return Box::new(AShape {
                vertices: vertices,
                normals: normals
            });
        }

        fn centroid(&self) -> [f32; 3] {
            let vertices = self.get_vertices();
            let n = vertices.len();
            let mut sum: [f32; 3] = [0.0; 3];
            for vertex in vertices {
                for i in 0..3 {
                    sum[i] += vertex.position[i];
                }
            }
            return [sum[0]/(n as f32), sum[1]/(n as f32), sum[2]/(n as f32)];
        }
    }
    pub struct AShape {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Shape3D for AShape {
        fn get_vertices(&self) -> Vec<Vertex> {
            return self.vertices.to_vec();
        }

        fn get_normals(&self) -> Vec<Normal> {
            return self.normals.to_vec();
        }
    }

    pub struct Quad {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Quad {
        pub fn new(bottom_left: [f32; 3], plane: [[f32; 3]; 2], material_id: i32) -> Quad {
            let bl = bottom_left;
            let rv = plane[0];
            let lv = plane[1];
            return Quad {
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
        fn get_vertices(&self) -> Vec<Vertex> {
            return self.vertices.to_vec();
        }

        fn get_normals(&self) -> Vec<Normal> {
            return self.normals.to_vec();
        }
    }

    pub struct Cube {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
        center: [f32; 3],
    }

    impl Cube {
        pub fn new(bottom_front_left: [f32; 3], side_length: f32, material_id: i32) -> Cube {

            let sl = side_length;
            let bfl = bottom_front_left;

            let top =       Quad::new([bfl[0],          bfl[1] + sl,    bfl[2]],        [[sl, 0.0, 0.0], [0.0, 0.0, sl]], material_id); // Top
            let bottom =    Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],        [[sl, 0.0, 0.0], [0.0, 0.0, -sl]], material_id); // Bottom
            let front =     Quad::new([bfl[0],          bfl[1],         bfl[2]],        [[sl, 0.0, 0.0], [0.0, sl, 0.0]], material_id); // Front
            let rear =      Quad::new([bfl[0] + sl,     bfl[1],         bfl[2] + sl],   [[-sl, 0.0, 0.0], [0.0, sl, 0.0]], material_id); // Rear
            let left =      Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],   [[0.0, 0.0, -sl], [0.0, sl, 0.0]], material_id); // Left
            let right =     Quad::new([bfl[0] + sl,     bfl[1],         bfl[2]],        [[0.0, 0.0, sl], [0.0, sl, 0.0]], material_id); // Right

            let quads: Vec<&dyn Shape3D> = vec! [&top, &bottom, &front, &rear, &left, &right];
            let cube = combine_shapes(&quads);

            return Cube {
                vertices: cube.get_vertices(),
                normals: cube.get_normals(),
                center: [bfl[0] + sl/2.0, bfl[1] + sl/2.0, bfl[2] + sl/2.0],
            }

        }

        pub fn center(&self) -> [f32; 3] {
            return self.center;
        }
    }

    impl Shape3D for Cube {
        fn get_vertices(&self) -> Vec<Vertex> {
            return self.vertices.to_vec();
        }

        fn get_normals(&self) -> Vec<Normal> {
            return self.normals.to_vec();
        }
    }

    pub fn combine_shapes(shapes: &Vec<&dyn Shape3D>) -> impl Shape3D {

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();

        for i in 0..shapes.len() {
            let shape: &dyn Shape3D = shapes[i];
            let s_vertices = shape.get_vertices();
            let s_normals = shape.get_normals();

            for j in 0..s_vertices.len() {
                vertices.push(s_vertices[j]);
                normals.push(s_normals[j]);
            }
            
        }

        return AShape {
            vertices: vertices,
            normals: normals,
        }
    }

}