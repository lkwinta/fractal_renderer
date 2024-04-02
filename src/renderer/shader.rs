use std::ffi::CString;
use gl::types::{GLchar, GLenum, GLuint};
use crate::resources::Resources;
use crate::renderer::Error;

pub struct Shader {
    shader_id: GLuint,
}

impl Shader{
    #[allow(dead_code)]
    pub fn from_resources(res: &Resources, res_name: &str, shader_type: u32) -> Result<Shader, Error> {
        let shader_resource = res.load_string(res_name).map_err(|e| Error::ResourceLoad {
            name: res_name.into(),
            inner: e,
        });

        if shader_resource.is_err() {
            return Err(shader_resource.unwrap_err());
        }
        let source = CString::new(shader_resource.unwrap()).unwrap();

        let shader_id = Self::compile_shader(&source, shader_type);

        match shader_id {
            Ok(shader_id) => Ok(Shader {
                shader_id,
            }),
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn from_source(source: &CString, shader_type: u32) -> Result<Shader, Error> {
        let shader_id = Self::compile_shader(source, shader_type);

        match shader_id {
            Ok(shader_id) => Ok(Shader {
                shader_id,
            }),
            Err(e) => Err(e),
        }
    }

    fn compile_shader(source: &CString, shader_type: GLenum) -> Result<GLuint, Error> {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            if shader == 0 {
                return Err(Error::CompileError {
                    name: "Unknown".into(),
                    message: "Failed to create shader".into(),
                });
            }

            gl::ShaderSource(shader, 1, &source.as_c_str().as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success = 1;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                let mut buffer = vec![0; len as usize];
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);

                let message = String::from_utf8(buffer).unwrap();
                return Err(Error::CompileError {
                    name: "Unknown".into(),
                    message,
                });
            }

            Ok(shader)
        }
    }

    pub fn id(&self) -> GLuint {
        self.shader_id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}