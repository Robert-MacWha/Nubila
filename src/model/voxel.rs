use cgmath::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    position: Point3<u32>,
    color: Point3<u8>,
}

impl Voxel {
    pub fn new(position: Point3<u32>, color: Point3<u8>) -> Voxel {
        Voxel { position, color }
    }

    pub fn position(&self) -> Point3<u32> {
        return self.position;
    }

    pub fn set_position(&mut self, pos: Point3<u32>) {
        self.position = pos;
    }

    pub fn color(&self) -> Point3<u8> {
        return self.color;
    }

    pub fn material(&self) -> u32 {
        let mat = (self.color.x as u32) << 16 | (self.color.y as u32) << 8 | self.color.z as u32;
        return mat;
    }
}
