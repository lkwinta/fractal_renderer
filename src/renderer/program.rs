use std::ffi::CStr;
use crate::renderer::{Error, Shader};
use gl;
use gl::types::{GLchar, GLuint};

pub struct Program {
    pub program_id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Program, Error> {
        unsafe {
            let id= gl::CreateProgram();

            if id == 0 {
                return Err(Error::LinkError {
                    name: "Unknown".into(),
                    message: "Failed to create program".into(),
                });
            }

            for shader in shaders {
                gl::AttachShader(id, shader.id());
            }

            gl::LinkProgram(id);

            let mut status = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            if status == 0 {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

                let mut buffer = vec![0u8; len as usize];
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);

                let message = String::from_utf8(buffer).unwrap();
                return Err(Error::LinkError {
                    name: "Unknown".into(),
                    message,
                });
            }

            for shader in shaders {
                gl::DetachShader(id, shader.id());
            }

            Ok(Program {
                program_id: id,
            })
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> GLuint {
        self.program_id
    }

    #[allow(dead_code)]
    pub fn set_f32(&self, name: &CStr, x: f32) -> Result<(), Error> {
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.to_bytes_with_nul().as_ptr() as *const GLchar);
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            gl::Uniform1f(location, x);

            Ok(())
        }
    }

    #[allow(dead_code)]
    pub fn set_i32(&self, name: &CStr, x: i32) -> Result<(), Error> {
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.to_bytes_with_nul().as_ptr() as *const GLchar);
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            gl::Uniform1i(location, x);

            Ok(())
        }
    }

    #[allow(dead_code)]
    pub fn set_f32_2(&self, name: &CStr, x: f32, y: f32) -> Result<(), Error> {
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.to_bytes_with_nul().as_ptr() as *const GLchar);
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            gl::Uniform2f(location, x, y);

            Ok(())
        }
    }

    #[allow(dead_code)]
    pub fn set_bool(&self, name: &CStr, x: bool) -> Result<(), Error> {
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, name.to_bytes_with_nul().as_ptr() as *const GLchar);
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            gl::Uniform1i(location, x as i32);

            Ok(())
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}