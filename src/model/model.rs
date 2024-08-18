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

        // Extract vertex data
        let vertex = &ply.payload["vertex"];
        let mut vmin: Option<Point3<u32>> = None;

        for v in vertex.iter() {
            let x = prop_to_f32(&v["x"])? as u32;
            let y = prop_to_f32(&v["y"])? as u32;
            let z = prop_to_f32(&v["z"])? as u32;
            let r = prop_to_uchar(&v["red"])?;
            let g = prop_to_uchar(&v["green"])?;
            let b = prop_to_uchar(&v["blue"])?;

            match &vmin {
                Some(min) => {
                    let x = x.min(min.x);
                    let y = y.min(min.y);
                    let z = z.min(min.z);
                    vmin = Some(Point3::new(x, y, z));
                }
                None => {
                    vmin = Some(Point3::new(x, y, z));
                }
            }

            // TODO: Assign material correctly
            let voxel = Voxel::new(Point3::new(x, y, z), 1);
            self.voxels.push(voxel);
        }

        // Normalize voxel positions
        let vmin = vmin.unwrap();
        let mut vmax = Point3::new(0, 0, 0);

        for voxel in self.voxels.iter_mut() {
            let pos = voxel.position();
            let x = pos.x - vmin.x;
            let y = pos.y - vmin.y;
            let z = pos.z - vmin.z;

            vmax.x = vmax.x.max(x);
            vmax.y = vmax.y.max(y);
            vmax.z = vmax.z.max(z);

            voxel.set_position(Point3::new(x, y, z));
        }

        self.size = Vector3::new(vmax.x + 1, vmax.y + 1, vmax.z + 1);

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
