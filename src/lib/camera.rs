pub struct LookAtCamera {
    pub aspect_ratio: f32,
    pub fovy: f32,
    pub position: cgmath::Point3<f32>,
    pub look_at: cgmath::Point3<f32>,
}

impl LookAtCamera {
    pub fn new(aspect_ratio: f32, fovy: f32, position: cgmath::Point3<f32>, look_at: cgmath::Point3<f32>) -> Self {
        Self { aspect_ratio, fovy, position, look_at }
    }

    pub fn generate_matrix(&self) -> cgmath::Matrix4<f32> {
        let projection = cgmath::perspective(
            cgmath::Deg(self.fovy), 
            self.aspect_ratio, 
            1.0, 
            10.0
        );
        let view = cgmath::Matrix4::look_at(
            self.position, 
            self.look_at, 
            cgmath::Vector3::unit_z(),
        );
        super::util::OPENGL_TO_WGPU_MATRIX * projection * view
    }
}