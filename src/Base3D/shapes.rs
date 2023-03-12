
use crate::Base3D::vertex3d::Vertex;
use crate::Base3D::normal3d::Normal;
use crate::Base3D::point_transform3d::*;
use crate::Base3D::shape3d::*;

#[derive(Clone)]
pub struct AShape {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
}

impl Shape3D for AShape {

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

        let top: AShape =       Quad::new([bfl[0],          bfl[1] + sl,    bfl[2]],        [[sl, 0.0, 0.0],    [0.0, 0.0, sl]], material_id); // Top
        let bottom: AShape =    Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],   [[sl, 0.0, 0.0],    [0.0, 0.0, -sl]], material_id); // Bottom
        let front: AShape =     Quad::new([bfl[0],          bfl[1],         bfl[2]],        [[sl, 0.0, 0.0],    [0.0, sl, 0.0]], material_id); // Front
        let rear: AShape =      Quad::new([bfl[0] + sl,     bfl[1],         bfl[2] + sl],   [[-sl, 0.0, 0.0],   [0.0, sl, 0.0]], material_id); // Rear
        let left: AShape =      Quad::new([bfl[0],          bfl[1],         bfl[2] + sl],   [[0.0, 0.0, -sl],   [0.0, sl, 0.0]], material_id); // Left
        let right: AShape =     Quad::new([bfl[0] + sl,     bfl[1],         bfl[2]],        [[0.0, 0.0, sl],    [0.0, sl, 0.0]], material_id); // Right

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

pub struct Icosahedron {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
}

impl Icosahedron {
    pub fn new(centroid: [f32; 3], radius: f32, material_id: i32) -> AShape {

        // Generating points in a magical way
        let phi: f32 = 0.5f32 + 5.0f32.sqrt()*0.5f32;
        let v0: f32 = 1f32;
        let v1: f32 = 1f32 / phi;

        let p1  = [0f32, v1, -v0];
        let p2  = [v1, v0, 0f32];
        let p3  = [-v1, v0, 0f32];
        let p4  = [0f32, v1, v0];
        let p5  = [0f32, -v1, v0];
        let p6  = [-v0, 0f32, v1];
        let p7  = [0f32, -v1, -v0];
        let p8  = [v0, 0f32, -v1];
        let p9  = [v0, 0f32, v1];
        let p10 = [-v0, 0f32, -v1];
        let p11 = [v1, -v0, 0f32];
        let p12 = [-v1, -v0, 0f32];

        // Generate all triangles
        let t0: AShape = Triangle::new([p1, p2, p3], material_id);
        let t1: AShape = Triangle::new([p4, p3, p2], material_id);
        let t2: AShape = Triangle::new([p4, p5, p6], material_id);
        let t3: AShape = Triangle::new([p4, p9, p5], material_id);
        let t4: AShape = Triangle::new([p1, p7, p8], material_id);
        let t5: AShape = Triangle::new([p1, p10, p7], material_id);
        let t6: AShape = Triangle::new([p5, p11, p12], material_id);
        let t7: AShape = Triangle::new([p7, p12, p11], material_id);
        let t8: AShape = Triangle::new([p3, p6, p10], material_id);
        let t9: AShape = Triangle::new([p12, p10, p6], material_id);
        let t10: AShape = Triangle::new([p2, p8, p9], material_id);
        let t11: AShape = Triangle::new([p11, p9, p8], material_id);
        let t12: AShape = Triangle::new([p4, p6, p3], material_id);
        let t13: AShape = Triangle::new([p4, p2, p9], material_id);
        let t14: AShape = Triangle::new([p1, p3, p10], material_id);
        let t15: AShape = Triangle::new([p1, p8, p2], material_id);
        let t16: AShape = Triangle::new([p7, p10, p12], material_id);
        let t17: AShape = Triangle::new([p7, p11, p8], material_id);
        let t18: AShape = Triangle::new([p5, p12, p6], material_id);
        let t19: AShape = Triangle::new([p5, p9, p11], material_id);

        // Combine all triangles to a whole shape, then scale and translate
        let mut shape: AShape = combine_shapes(vec![&t0, &t1, &t2, &t3, &t4, &t5, &t6, &t7, &t8, &t9, &t10, &t11, &t12, &t13, &t14, &t15, &t16, &t17, &t18, &t19]);
        shape.scale_mut_O(radius);
        shape.translate_mut(centroid);

        return shape;

    }
}

impl Shape3D for Icosahedron {
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

        let mut icosahedron = Icosahedron::new(centroid, radius, material_id);

        for i in 0..precision {
            icosahedron.subdivide_mut();
            icosahedron.spherify_mut(radius);
        }
        
        return icosahedron;
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