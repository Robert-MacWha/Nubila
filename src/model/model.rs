use std::{
    fs,
    io::{BufRead, BufReader},
};

use cgmath::{Point3, Vector3};

use super::voxel::Voxel;

pub struct Model {
    path: String,
    voxels: Vec<Voxel>,
    size: Vector3<u32>,
}

struct PlyVoxel {
    x: i32,
    y: i32,
    z: i32,
    r: u8,
    g: u8,
    b: u8,
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

        //* */ Read PLY file
        let start = std::time::Instant::now();
        let contents = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        let mut lines = contents.lines();
        log::info!("Opened file: elapsed={:?}", start.elapsed());

        let magic_number = lines.next().ok_or("missing magic number")?;
        if magic_number != "ply" {
            return Err("Invalid PLY file".to_string());
        }

        let format = lines.next().ok_or("missing format")?;
        if format != "format ascii 1.0" {
            return Err(format!("Invalid PLY format: {}", format));
        }

        // Parse header
        let start = std::time::Instant::now();
        let mut vertex_count = 0;
        for line in lines.by_ref() {
            if line == "end_header" {
                break;
            }

            if line.starts_with("element vertex") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                vertex_count = parts[2].parse::<u32>().map_err(|e| e.to_string())?;
            }
        }
        log::info!("Parsed header: elapsed={:?}", start.elapsed());

        //* Parse file data
        let mut ply_voxel: Vec<PlyVoxel> = Vec::with_capacity(vertex_count as usize);
        let mut min = Vector3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Vector3::new(i32::MIN, i32::MIN, i32::MIN);

        let start = std::time::Instant::now();
        for line in lines {
            // Skip comments
            if line.starts_with("comment") || line.is_empty() {
                continue;
            }

            // injest vertex data
            let vertex = PlyVoxel::new(line)?;

            // Update min and max
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);

            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);

            // Append to vertices
            ply_voxel.push(vertex);
        }
        log::info!("Parsed data: elapsed={:?}", start.elapsed());

        let start = std::time::Instant::now();
        self.voxels.reserve(ply_voxel.len());
        //* Convet to model
        for vertex in ply_voxel {
            let x = (vertex.x - min.x) as u32;
            let y = (vertex.y - min.y) as u32;
            let z = (vertex.z - min.z) as u32;

            let voxel = Voxel::new(Point3::new(x, y, z), vertex.r, vertex.g, vertex.b);
            self.voxels.push(voxel);
        }
        log::info!("Converted to model: elapsed={:?}", start.elapsed());

        self.size = Vector3::new(
            (max.x - min.x) as u32,
            (max.y - min.y) as u32,
            (max.z - min.z) as u32,
        );

        return Ok(());
    }
}

impl PlyVoxel {
    fn new(line: &str) -> Result<PlyVoxel, String> {
        let mut parts = line.split_whitespace();
        let x = parts
            .next()
            .ok_or("missing x")?
            .parse::<i32>()
            .unwrap_or_default();

        //? For whatever reason, magica voxel has the y and z axis swapped
        let z = parts
            .next()
            .ok_or("missing z")?
            .parse::<i32>()
            .unwrap_or_default();
        let y = parts
            .next()
            .ok_or("missing y")?
            .parse::<i32>()
            .unwrap_or_default();
        let r = parts
            .next()
            .ok_or("missing r")?
            .parse::<u8>()
            .unwrap_or_default();
        let g = parts
            .next()
            .ok_or("missing g")?
            .parse::<u8>()
            .unwrap_or_default();
        let b = parts
            .next()
            .ok_or("missing b")?
            .parse::<u8>()
            .unwrap_or_default();

        Ok(PlyVoxel { x, y, z, r, g, b })
    }
}
