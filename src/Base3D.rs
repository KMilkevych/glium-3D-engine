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

    fn plength(p: [f32; 3]) -> f32 {
        return (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
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
        fn get_vertices(&self) -> &Vec<Vertex>;
        fn get_normals(&self) -> &Vec<Normal>;

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex>;
        fn get_mut_normals(&mut self) -> &mut Vec<Normal>;

        fn rotate(&self, angle_XYZ: [f32; 3]) -> AShape {

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

        fn rotate_O(&self, angle_XYZ: [f32; 3]) -> AShape {
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

        fn rotate_mut(&mut self, angle_XYZ: [f32; 3]) {

            let mut vertices: Vec<Vertex> = Vec::new();
            let mut normals: Vec<Normal> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                vertices.push(vertex.rotate(angle_XYZ));
            }
    
            for normal in self.get_normals().iter() {
                normals.push(normal.rotate(angle_XYZ));
            }
    
           *self.get_mut_vertices() = vertices;
           *self.get_mut_normals() = normals;
        }

        fn rotate_mut_O(&mut self, angle_XYZ: [f32; 3]){
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
    
            *self.get_mut_vertices() = vertices;
            *self.get_mut_normals() = normals;
        }

        fn translate(&self, relative_XYZ: [f32; 3]) -> AShape {
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

        fn translate_mut(&mut self, relative_XYZ: [f32; 3]) {
            let mut vertices: Vec<Vertex> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                let v: Vertex = vertex.translate(relative_XYZ);
                vertices.push(v);
            }
    
            *self.get_mut_vertices() = vertices;
        }

        fn scale_O(&self, factor: f32) -> AShape {
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

        fn scale_mut_O(&mut self, factor: f32) {
            let origin: [f32; 3] = self.centroid();
            let mut vertices: Vec<Vertex> = Vec::new();
    
            for vertex in self.get_vertices().iter() {
                let v: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
                let v: Vertex = v.scale(factor);
                let v: Vertex = v.translate(origin);
                vertices.push(v);
            }

            *self.get_mut_vertices() = vertices;
        }

        fn centroid(&self) -> [f32; 3] {
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

        fn subdivide_mut(&mut self) {

            // Subdivide all triangles
            let vertices: Vec<Vertex> = self.get_vertices().clone();
            let normals: Vec<Normal> = self.get_normals().clone();

            let mut subshapes: Vec<AShape> = Vec::new();

            for j in 0..(self.get_vertices().len() / 3) {

                let v1: Vertex = *vertices.get(j*3).unwrap();
                let v2: Vertex = *vertices.get(j*3+1).unwrap();
                let v3: Vertex = *vertices.get(j*3+2).unwrap();

                let v1p: [f32; 3] = v1.position;
                let v2p: [f32; 3] = v2.position;
                let v3p: [f32; 3] = v3.position;

                let normal: Normal = *normals.get(j*3).unwrap();

                // Find in-between points
                let v1v2: Vertex = Vertex{ 
                    position: [(v1p[0] + v2p[0])*0.5f32, (v1p[1] + v2p[1])*0.5f32, (v1p[2] + v2p[2])*0.5f32],
                    material_id: v1.material_id,
                    texture: [(v1.texture[0] + v2.texture[0])*0.5f32, (v1.texture[1] + v2.texture[1])*0.5f32],
                };

                let v3v2: Vertex = Vertex{ 
                    position: [(v3p[0] + v2p[0])*0.5f32, (v3p[1] + v2p[1])*0.5f32, (v3p[2] + v2p[2])*0.5f32],
                    material_id: v3.material_id,
                    texture: [(v3.texture[0] + v2.texture[0])*0.5f32, (v3.texture[1] + v2.texture[1])*0.5f32],
                };

                let v1v3: Vertex = Vertex{ 
                    position: [(v1p[0] + v3p[0])*0.5f32, (v1p[1] + v3p[1])*0.5f32, (v1p[2] + v3p[2])*0.5f32],
                    material_id: v1.material_id,
                    texture: [(v1.texture[0] + v3.texture[0])*0.5f32, (v1.texture[1] + v3.texture[1])*0.5f32],
                };

                // Create new subdivided triangles
                let t1_12_13 = AShape {
                    vertices: vec![v1.clone(), v1v2.clone(), v1v3.clone()],
                    normals: vec![normal; 3],
                };

                let t12_2_32 = AShape {
                    vertices: vec![v1v2.clone(), v2.clone(), v3v2.clone()],
                    normals: vec![normal; 3],
                };

                let t13_32_3 = AShape {
                    vertices: vec![v1v3.clone(), v3v2.clone(), v3.clone()],
                    normals: vec![normal; 3],
                };

                let t12_32_13 = AShape {
                    vertices: vec![v1v2.clone(), v3v2.clone(), v1v3.clone()],
                    normals: vec![normal; 3],
                };

                // Push to list of subshapes
                subshapes.push(t1_12_13);
                subshapes.push(t12_2_32);
                subshapes.push(t13_32_3);
                subshapes.push(t12_32_13);

            }

            // Reassign the shape
            let subdivided = combine_shapes(subshapes.iter().collect::<Vec::<&AShape>>());
            *self.get_mut_vertices() = subdivided.get_vertices().to_owned();
            *self.get_mut_normals() = subdivided.get_normals().to_owned();
            
        }

        fn spherify(&self, radius: f32) -> AShape {
            let centroid = self.centroid();

            let mut vertices: Vec<Vertex> = Vec::new();
            let mut normals: Vec<Normal> = self.get_normals().clone();

            for vertex in self.get_vertices() {
                let pv = vertex.position;
                let diff = [pv[0] - centroid[0], pv[1] - centroid[1], pv[2] - centroid[2]];
                vertices.push(Vertex {
                    position: ptranslate(centroid, pscale(diff, (radius/plength(diff)))),
                    material_id: vertex.material_id,
                    texture: vertex.texture
                });
            }

            return AShape { vertices: vertices, normals: normals };
        }

        fn spherify_mut(&mut self, radius: f32)  {

            let centroid = self.centroid();

            for i in 0..self.get_vertices().len() {
                let pv = self.get_vertices().get(i).unwrap().position;
                let diff = [pv[0] - centroid[0], pv[1] - centroid[1], pv[2] - centroid[2]];

                let newp = ptranslate(centroid, pscale(diff, (radius/plength(diff))));
                let newn = pscale(diff, -(1f32/plength(diff)));

                self.get_mut_vertices().get_mut(i).unwrap().position = newp;
                self.get_mut_normals().get_mut(i).unwrap().normal = newn;
            }
            

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

        fn get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_mut_normals(&mut self) -> &mut Vec<Normal> {
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
        fn get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_mut_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub struct Triangle {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Triangle {
        pub fn new(points: [[f32; 3]; 3], material_id: i32) -> AShape {
            let mut vertices: Vec<Vertex> = Vec::new();
            vertices.push(Vertex {
                position: points[0],
                texture: [0f32, 1f32],
                material_id: material_id
            });
            vertices.push(Vertex {
                position: points[1],
                texture: [1f32, 1f32],
                material_id: material_id
            });
            vertices.push(Vertex {
                position: points[2],
                texture: [0.5f32, 0f32],
                material_id: material_id
            });

            let mut normals: Vec<Normal> = Vec::new();
            let norm = Normal::normal([[points[1][0] - points[0][0], points[1][1] - points[0][1], points[1][2] - points[0][2]], [points[2][0] - points[0][0], points[2][1] - points[0][1], points[2][2] - points[0][2]]]);
            normals.push(norm);
            normals.push(norm);
            normals.push(norm);

            return AShape {
                vertices: vertices,
                normals: normals
            };
        }
    }

    impl Shape3D for Triangle {
        fn get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_mut_normals(&mut self) -> &mut Vec<Normal> {
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

            let top: AShape =       Quad::new([bfl[0],          bfl[1] + sl,    bfl[2]],        [[sl, 0.0, 0.0], [0.0, 0.0, sl]], material_id); // Top
            let bottom: AShape =    Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],        [[sl, 0.0, 0.0], [0.0, 0.0, -sl]], material_id); // Bottom
            let front: AShape =     Quad::new([bfl[0],          bfl[1],         bfl[2]],        [[sl, 0.0, 0.0], [0.0, sl, 0.0]], material_id); // Front
            let rear: AShape =      Quad::new([bfl[0] + sl,     bfl[1],         bfl[2] + sl],   [[-sl, 0.0, 0.0], [0.0, sl, 0.0]], material_id); // Rear
            let left: AShape =      Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],   [[0.0, 0.0, -sl], [0.0, sl, 0.0]], material_id); // Left
            let right: AShape =     Quad::new([bfl[0] + sl,     bfl[1],         bfl[2]],        [[0.0, 0.0, sl], [0.0, sl, 0.0]], material_id); // Right

            let quads: Vec<&AShape> = vec! [&top, &bottom, &front, &rear, &left, &right];
            let cube = combine_shapes(quads);

            return AShape {
                vertices: cube.get_vertices().to_owned(),
                normals: cube.get_normals().to_owned(),
            }
        }
    }

    impl Shape3D for Cube {
        fn get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_mut_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub struct Tetrahedron {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Tetrahedron {
        pub fn new(centroid: [f32; 3], radius: f32, material_id: i32) -> AShape {

            // First triangle:
            // bottom left: left of centr. bottom of centr. forward of centr.
            // bottom right: right of centr. bottom of centr. forward of centr.
            // top: centr, up of centr, centr

            let base: [f32; 3] = [(0.25f32).asin().cos(), -0.25f32, 0f32];

            let p0: [f32; 3] = protate_Y(base, (150f32).to_radians());
            let p0: [f32; 3] = ptranslate(pscale(p0, radius), centroid);

            let p1: [f32; 3] = protate_Y(base, (30f32).to_radians());
            let p1: [f32; 3] = ptranslate(pscale(p1, radius), centroid);

            let p2: [f32; 3] = protate_Y(base, (-90f32).to_radians());
            let p2: [f32; 3] = ptranslate(pscale(p2, radius), centroid);

            let p3: [f32; 3] = ptranslate(pscale([0f32, 1f32, 0f32], radius), centroid);

            let t0: AShape = Triangle::new(
                [p0, p1, p3],
                material_id,
            );

            let t1: AShape = Triangle::new(
                [p1, p2, p3],
                material_id,
            );

            let t2: AShape = Triangle::new(
                [p2, p0, p3],
                material_id,
            );

            let t3: AShape = Triangle::new(
                [p0, p2, p1],
                material_id,
            );

            return combine_shapes(vec![&t0, &t1, &t2, &t3]);
        }
    }

    impl Shape3D for Tetrahedron {
        fn get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_mut_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub struct Sphere {
        pub vertices: Vec<Vertex>,
        pub normals: Vec<Normal>,
    }

    impl Sphere {
        pub fn new(centroid: [f32; 3], radius: f32, precision: i32, material_id: i32) -> AShape {

            let mut tetrahedron = Tetrahedron::new(centroid, radius, material_id);

            for i in 0..precision {
                tetrahedron.subdivide_mut();
                tetrahedron.spherify_mut(radius);
            }
            
            /*
            let mut tetrahedron_2 = Tetrahedron::new(ptranslate(centroid, [0.2f32, 0f32, 0f32]), radius, material_id);

            for i in 0..precision {
                tetrahedron_2.subdivide_mut();
            }
            tetrahedron_2.spherify_mut(radius);
            
            return combine_shapes(vec![&tetrahedron, &tetrahedron_2]);
            */

            return tetrahedron;
        }
    }

    impl Shape3D for Sphere {
        fn get_vertices(&self) -> &Vec<Vertex> {
            return &self.vertices;
        }

        fn get_normals(&self) -> &Vec<Normal> {
            return &self.normals;
        }

        fn get_mut_vertices(&mut self) -> &mut Vec<Vertex> {
            return &mut self.vertices;
        }

        fn get_mut_normals(&mut self) -> &mut Vec<Normal> {
            return &mut self.normals;
        }
    }

    pub fn combine_shapes(shapes: Vec<&AShape>) -> AShape {

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();

        for i in 0..shapes.len() {
            let shape: &AShape = shapes[i];
            
            vertices.extend(shape.get_vertices().to_owned().iter());
            normals.extend(shape.get_normals().to_owned().iter());
        }

        return AShape {
            vertices: vertices,
            normals: normals,
        }
    }

}