use std::{any::Any, error::Error, fs};

use cgmath::{Point3, Vector3};
use ply_rs::ply;

use super::voxel::Voxel;

pub struct Model {
    path: String,
    voxels: Vec<Voxel>,
    size: Vector3<u32>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let voxels = Vec::new();
        let size = Vector3::new(0, 0, 0);

        let mut model = Model {
            path: String::from(path),
            size,
            voxels,
        };

        model.load().expect("Failed to load model");

        return model;
    }

    pub fn size(&self) -> Vector3<u32> {
        self.size
    }

    pub fn voxels(&self) -> &Vec<Voxel> {
        &self.voxels
    }

    pub fn reload(&mut self) {
        match self.load() {
            Ok(_) => println!("Model reloaded"),
            Err(e) => eprintln!("Failed to reload model: {}", e),
        }
    }

    /// Load a model from a file.
    /// Only supports PLY files with ASCII format.
    fn load(&mut self) -> Result<(), String> {
        self.voxels.clear();

        // Read PLY file
        let mut f = fs::File::open(&self.path).map_err(|e| e.to_string())?;
        let p = ply_rs::parser::Parser::<ply::DefaultElement>::new();
        let ply = p.read_ply(&mut f).map_err(|e| e.to_string())?;
        if !ply.payload.contains_key("vertex") {
            return Err("missing vertex data".to_string());
        }

        // find min and max of vertex positions
        let mut min = Point3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Point3::new(i32::MIN, i32::MIN, i32::MIN);

        for v in ply.payload["vertex"].iter() {
            let x = prop_to_f32(&v["x"])? as i32;
            let y = prop_to_f32(&v["y"])? as i32;
            let z = prop_to_f32(&v["z"])? as i32;

            min.x = min.x.min(x);
            min.y = min.y.min(y);
            min.z = min.z.min(z);
            max.x = max.x.max(x);
            max.y = max.y.max(y);
            max.z = max.z.max(z);
        }

        // Extract vertex data
        for v in ply.payload["vertex"].iter() {
            let x = ((prop_to_f32(&v["x"])? as i32) - min.x) as u32;
            let y = ((prop_to_f32(&v["y"])? as i32) - min.y) as u32;
            let z = ((prop_to_f32(&v["z"])? as i32) - min.z) as u32;
            let r = prop_to_uchar(&v["red"])?;
            let g = prop_to_uchar(&v["green"])?;
            let b = prop_to_uchar(&v["blue"])?;

            // TODO: Assign material correctly
            let voxel = Voxel::new(Point3::new(x, y, z), 1);
            self.voxels.push(voxel);
        }

        self.size = Vector3::new(
            (max.x - min.x + 1) as u32,
            (max.y - min.y + 1) as u32,
            (max.z - min.z + 1) as u32,
        );

        Ok(())
    }
}

fn prop_to_f32(prop: &ply::Property) -> Result<f32, String> {
    match prop {
        ply::Property::Float(x) => Ok(*x),
        _ => Err("Invalid f32 prop".to_string()),
    }
}

fn prop_to_uchar(prop: &ply::Property) -> Result<u8, String> {
    match prop {
        ply::Property::UChar(x) => Ok(*x),
        _ => Err("Invalid f32 prop".to_string()),
    }
}
