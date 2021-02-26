unsafe fn create_shader_from_file(src: &str, shader_type: gl::types::GLenum) -> u32 {
    let shader = gl::CreateShader(shader_type);

    let shader_string =
        std::fs::read_to_string(src).expect((std::format!("arquivo {}", src)).as_str());
    let shader_str = shader_string.as_str();
    let shader_source = std::ffi::CString::new(shader_str.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &shader_source.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);
    return shader;
}

unsafe fn create_shader_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    return shader_program;
}

pub struct GLShaderProgram {
    pub program_id: u32,
}

impl GLShaderProgram {
    pub fn new_from_file(vertex_shader_path: &str, fragment_shader_path: &str) -> GLShaderProgram {
        unsafe {
            let shader_program: u32;
            let vs_id = create_shader_from_file(vertex_shader_path, gl::VERTEX_SHADER);
            let fs_id = create_shader_from_file(fragment_shader_path, gl::FRAGMENT_SHADER);
            shader_program = create_shader_program(vs_id, fs_id);
            // gl::DeleteShader(vs_id);
            // gl::DeleteShader(fs_id);
            return GLShaderProgram {
                program_id: shader_program,
            };
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
    pub fn set_u3f(&self, x: f32, y: f32, z: f32, uniform_name: &str) {
        unsafe {
            let uniform_name_cstr = std::ffi::CString::new(uniform_name).unwrap();
            let uniform_location =
                gl::GetUniformLocation(self.program_id, uniform_name_cstr.as_ptr());
            gl::Uniform3f(uniform_location, x, y, z);
        }
    }
}
