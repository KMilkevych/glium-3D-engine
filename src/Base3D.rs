pub mod General {
    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 3],
        pub texture: [f32; 2],
        pub material_id: i32,
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
    }
    implement_vertex!(Normal, normal);

    /*
     * General shapes 
     */

    pub trait Shape3D {
        fn get_vertices(&self) -> Vec<Vertex>;
        fn get_normals(&self) -> Vec<Normal>;
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

}