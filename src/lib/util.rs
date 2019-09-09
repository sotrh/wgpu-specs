#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[allow(dead_code)]
pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::mem::size_of;
    use std::slice::from_raw_parts;

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

#[allow(dead_code)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    let ty = match stage {
        ShaderStage::Vertex => glsl_to_spirv::ShaderType::Vertex,
        ShaderStage::Fragment => glsl_to_spirv::ShaderType::Fragment,
        ShaderStage::Compute => glsl_to_spirv::ShaderType::Compute,
    };

    let compiled = glsl_to_spirv::compile(&code, ty).unwrap();
    wgpu::read_spirv(compiled).unwrap()
}

pub fn rand(min: f32, max: f32) -> f32 {
    assert!(min <= max);
    (max - min) * rand::random::<f32>() + min
}

pub fn rand_vec2(min: f32, max: f32) -> cgmath::Vector2<f32> {
    cgmath::Vector2::new(
        rand(min, max),
        rand(min, max),
    )
}

pub fn rand_vec3(min: f32, max: f32) -> cgmath::Vector3<f32> {
    cgmath::Vector3::new(
        rand(min, max),
        rand(min, max),
        rand(min, max),
    )
}

