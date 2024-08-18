use cgmath::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    position: Point3<u32>,
    material: u32,
}

impl Voxel {
    pub fn new(position: Point3<u32>, material: u32) -> Voxel {
        Voxel { position, material }
    }

    pub fn position(&self) -> Point3<u32> {
        self.position
    }

    pub fn set_position(&mut self, pos: Point3<u32>) {
        self.position = pos;
    }

    pub fn material(&self) -> u32 {
        self.material
    }
}
