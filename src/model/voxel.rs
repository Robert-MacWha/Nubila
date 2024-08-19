use cgmath::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    position: Point3<u32>,
    r: u8,
    g: u8,
    b: u8,
}

impl Voxel {
    pub fn new(position: Point3<u32>, r: u8, g: u8, b: u8) -> Voxel {
        Voxel { position, r, g, b }
    }

    pub fn position(&self) -> Point3<u32> {
        self.position
    }

    pub fn set_position(&mut self, pos: Point3<u32>) {
        self.position = pos;
    }

    pub fn material(&self) -> u32 {
        let mat = 1 << 31 | (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32;
        return mat;
    }
}
