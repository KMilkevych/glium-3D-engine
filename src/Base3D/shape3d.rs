use async_trait_fn::async_trait;

use crate::Base3D::vertex3d::Vertex;
use crate::Base3D::normal3d::Normal;
use crate::Base3D::point_transform3d::*;
use crate::Base3D::shapes::*;


/*
* General shapes 
*/
#[async_trait]
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
            let p: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
            let p: Vertex = p.rotate(angle_XYZ);
            let p: Vertex = p.translate(origin);
            vertices.push(p);
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
            let p: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
            let p: Vertex = p.rotate(angle_XYZ);
            let p: Vertex = p.translate(origin);
            vertices.push(p);
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
            let p: Vertex = vertex.translate(relative_XYZ);
            vertices.push(p);
        }

        return AShape {
            vertices: vertices,
            normals: self.get_normals().to_vec()
        };
    }

    fn translate_mut(&mut self, relative_XYZ: [f32; 3]) {
        let mut vertices: Vec<Vertex> = Vec::new();

        for vertex in self.get_vertices().iter() {
            let p: Vertex = vertex.translate(relative_XYZ);
            vertices.push(p);
        }

        *self.get_mut_vertices() = vertices;
    }

    fn scale_O(&self, factor: f32) -> AShape {
        let origin: [f32; 3] = self.centroid();
        let mut vertices: Vec<Vertex> = Vec::new();

        for vertex in self.get_vertices().iter() {
            let p: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
            let p: Vertex = p.scale(factor);
            let p: Vertex = p.translate(origin);
            vertices.push(p);
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
            let p: Vertex = vertex.translate([-origin[0], -origin[1], -origin[2]]);
            let p: Vertex = p.scale(factor);
            let p: Vertex = p.translate(origin);
            vertices.push(p);
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

        for i in 0..(self.get_vertices().len() / 3) {

            let v1: Vertex = *vertices.get(i*3).unwrap();
            let v2: Vertex = *vertices.get(i*3+1).unwrap();
            let v3: Vertex = *vertices.get(i*3+2).unwrap();

            let v1p: [f32; 3] = v1.position;
            let v2p: [f32; 3] = v2.position;
            let v3p: [f32; 3] = v3.position;

            let normal: Normal = *normals.get(i*3).unwrap();

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


    fn spherify_mut(&mut self, radius: f32)  {

        let centroid = self.centroid();
        
        for i in 0..(self.get_vertices().len()/3) {

            let mut newp: [[f32; 3]; 3] = [[0f32; 3]; 3];

            for j in 0..3 {
                let pv: [f32; 3] = self.get_vertices().get(3*i + j).unwrap().position;
                let diff: [f32; 3] = ptranslate(pv, pscale(centroid, -1f32));
                newp[j] = ptranslate(centroid, pscale(diff, radius/plength(diff)));
                self.get_mut_vertices().get_mut(3*i + j).unwrap().position = newp[j];
            }

            let norm = vcross(
                ptranslate(newp[1], pscale(newp[0], -1f32)),
                ptranslate(newp[2], pscale(newp[1], -1f32))
            );

            for j in 0..3 {
                self.get_mut_normals().get_mut(3*i + j).unwrap().normal = norm;
            }

        }
        

    }

}
