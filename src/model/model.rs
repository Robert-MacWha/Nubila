use std::fs::File;

use cgmath::{Point3, Vector3};
use linked_hash_map::LinkedHashMap;
use ply_rs::ply::Property;

use super::voxel::Voxel;

pub struct Model {
    path: String,
    voxels: Vec<Voxel>,

    size: Vector3<u32>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let model = Model::load_voxels(path).unwrap();
        return model;
    }

    pub fn reload(&mut self) {
        let model = Model::load_voxels(&self.path).unwrap();
        self.voxels = model.voxels;
        self.size = model.size;
    }

    pub fn size(&self) -> Vector3<u32> {
        return self.size;
    }

    pub fn voxels(&self) -> &Vec<Voxel> {
        return &self.voxels;
    }

    fn load_voxels(path: &str) -> Result<Model, String> {
        let mut f = File::open(path).expect("File not found");

        let p = ply_rs::parser::Parser::<ply_rs::ply::DefaultElement>::new();
        let ply = p.read_ply(&mut f).expect("Error reading PLY file");

        let vertice_count = ply.header.elements["vertex"].count;
        if vertice_count == 0 {
            return Err("No vertices found".to_string());
        }

        let ply_vertices = &ply.payload["vertex"];
        if ply_vertices.len() != vertice_count as usize {
            return Err("Wrong number of vertices".to_string());
        }

        struct Vertice {
            pos: Point3<f32>,
            color: Point3<u8>,
        }

        // collect raw vertices
        let mut floating_vertices: Vec<Vertice> = Vec::with_capacity(vertice_count);
        let mut min: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
        let mut max: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
        for vertice in ply_vertices {
            let x = Model::get_float_prop(vertice, "x").unwrap();
            let y = Model::get_float_prop(vertice, "y").unwrap();
            let z = -Model::get_float_prop(vertice, "z").unwrap(); //? Not sure why, but z is inverted
            let r = Model::get_char_prop(vertice, "red").unwrap_or(1);
            let g = Model::get_char_prop(vertice, "green").unwrap_or(1);
            let b = Model::get_char_prop(vertice, "blue").unwrap_or(1);

            min = Point3::new(min.x.min(x), min.y.min(y), min.z.min(z));
            max = Point3::new(max.x.max(x), max.y.max(y), max.z.max(z));

            floating_vertices.push(Vertice {
                pos: Point3::new(x, y, z),
                color: Point3::new(r, g, b),
            });
        }

        // Normalize to Voxels
        let mut voxels: Vec<Voxel> = Vec::with_capacity(vertice_count);
        let size: Vector3<u32> = Vector3::new(
            (max.x - min.x) as u32,
            (max.y - min.y) as u32,
            (max.z - min.z) as u32,
        );
        for vertice in floating_vertices {
            let position = Point3::new(
                (vertice.pos.x - min.x) as u32,
                (vertice.pos.y - min.y) as u32,
                (vertice.pos.z - min.z) as u32,
            );

            voxels.push(Voxel::new(position, vertice.color));
        }

        Ok(Model {
            path: path.to_string(),
            size,
            voxels,
        })
    }

    fn get_float_prop(vertice: &LinkedHashMap<String, Property>, key: &str) -> Result<f32, String> {
        match vertice.get(key) {
            Some(x) => match x {
                Property::Float(x) => Ok(*x),
                _ => Err(format!("Wrong type for {}", key)),
            },
            None => Err(format!("Missing {}", key)),
        }
    }

    fn get_char_prop(vertice: &LinkedHashMap<String, Property>, key: &str) -> Result<u8, String> {
        match vertice.get(key) {
            Some(x) => match x {
                Property::UChar(x) => Ok(*x),
                _ => Err(format!("Wrong type for {}", key)),
            },
            None => Err(format!("Missing {}", key)),
        }
    }
}
