use std::ffi::CStr;
use crate::renderer::{Error, Shader};
use gl;
use gl::types::{GLchar, GLuint};

/// Wrapper around an OpenGL shader program
/// hides all unsafe calls and provides a simple interface to set the uniforms
/// and use the program for rendering
/// also includes a drop implementation to delete the program when it goes out of scope
pub struct Program {
    pub program_id: GLuint,
}

macro_rules! set_shader_uniform {
    ($name:tt, $gl_function:expr, $($v:tt: $t:ty), +) => {
        #[allow(dead_code)]
        pub fn $name(&self, uniform_name: &CStr, $($v: $t), +) -> Result<(), Error> {
            let location = unsafe { gl::GetUniformLocation(self.program_id, uniform_name.to_bytes_with_nul().as_ptr() as *const GLchar) };
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: uniform_name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            unsafe { $gl_function(location, $($v), +) };
            Ok(())
        }
    };
    ($name:tt, $gl_function:expr, $($v:tt: $t:ty, $cast:ty), +) => {
        #[allow(dead_code)]
        pub fn $name(&self, uniform_name: &CStr, $($v: $t), +) -> Result<(), Error> {
            let location = unsafe { gl::GetUniformLocation(self.program_id, uniform_name.to_bytes_with_nul().as_ptr() as *const GLchar) };
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: uniform_name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            unsafe { $gl_function(location, $($v as $cast), +) };
            Ok(())
        }
    };
}

impl Program {
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Program, Error> {
        let id= unsafe {gl::CreateProgram()};

        if id == 0 {
            return Err(Error::LinkError {
                name: "Unknown".into(),
                message: "Failed to create program".into(),
            });
        }

        for shader in shaders {
            unsafe {gl::AttachShader(id, shader.id()) };
        }

        unsafe{ gl::LinkProgram(id) };

        let mut status = 0;
        unsafe{ gl::GetProgramiv(id, gl::LINK_STATUS, &mut status)};

        if status == 0 {
            let mut len = 0;
            unsafe{ gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) } ;

            let mut buffer = vec![0u8; len as usize];
            unsafe{ gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar)};

            let message = String::from_utf8(buffer).unwrap();
            return Err(Error::LinkError {
                name: "Unknown".into(),
                message,
            });
        }

        for shader in shaders {
            unsafe{ gl::DetachShader(id, shader.id())};
        }

        Ok(Program {
            program_id: id,
        })
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.program_id); }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> GLuint {
        self.program_id
    }

    set_shader_uniform!(set_f32, gl::Uniform1f, x: f32);
    set_shader_uniform!(set_i32, gl::Uniform1i, x: i32);
    set_shader_uniform!(set_f32_2, gl::Uniform2f, x: f32, y: f32);
    set_shader_uniform!(set_f32_3, gl::Uniform3f, x: f32, y: f32, z: f32);
    set_shader_uniform!(set_bool, gl::Uniform1i, x: bool, i32);
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program_id); }
    }
}