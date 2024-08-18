use cgmath::{
    perspective, Deg, InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3,
};

pub struct Camera {
    pos: Point3<f32>,
    dir: Vector3<f32>,
    fov: Rad<f32>,
    aspect_ratio: f32,

    view_matrix: Matrix4<f32>,
    proj_matrix: Matrix4<f32>,
}

const UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

impl Camera {
    pub fn new(fov: Deg<f32>, aspect_ratio: f32) -> Self {
        let view_matrix = Matrix4::identity();
        let proj_matrix = Matrix4::identity();

        let mut camera = Self {
            pos: Point3::new(0.0, 0.0, -1.0),
            dir: Vector3::new(0.0, 0.0, 1.0),
            fov: Rad::from(fov),
            aspect_ratio,
            view_matrix,
            proj_matrix,
        };

        camera.refresh_matrix();

        return camera;
    }

    pub fn position(&self) -> Point3<f32> {
        return self.pos;
    }

    pub fn direction(&self) -> Vector3<f32> {
        return self.dir;
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        return self.view_matrix;
    }

    pub fn proj_matrix(&self) -> Matrix4<f32> {
        return self.proj_matrix;
    }

    pub fn set_position(&mut self, pos: Point3<f32>) {
        self.pos = pos;
        self.refresh_matrix();
    }

    pub fn set_direction(&mut self, dir: Vector3<f32>) {
        self.dir = dir;
        self.refresh_matrix();
    }

    pub fn look_at(&mut self, target: Point3<f32>) {
        self.dir = (target - self.pos).normalize();
        self.refresh_matrix();
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.refresh_matrix();
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.pos += translation;
        self.refresh_matrix();
    }

    fn refresh_matrix(&mut self) {
        self.dir = self.dir.normalize();
        self.proj_matrix = perspective(self.fov, self.aspect_ratio, 0.01, 1000.0);
        self.view_matrix = Matrix4::look_to_rh(self.pos, self.dir, UP);
    }
}
