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

/// useful macro for generating the set_uniform functions
/// used to access variables defined inside shaders
/// in to variants allowing for parameterized types
macro_rules! set_shader_uniform {
    ($name:tt, $gl_function:expr, $($v:tt: $t:ty), +) => {
        /// set a uniform value in the shader program
        #[allow(dead_code)]
        pub fn $name(&self, uniform_name: &CStr, $($v: $t), +) -> Result<(), Error> {
            // try to get the location of the uniform
            let location = unsafe { gl::GetUniformLocation(self.program_id, uniform_name.to_bytes_with_nul().as_ptr() as *const GLchar) };
            // if the location is -1 the uniform was not found
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: uniform_name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            // set the uniform value
            unsafe { $gl_function(location, $($v), +) };
            Ok(())
        }
    };
    ($name:tt, $gl_function:expr, $($v:tt: $t:ty, $cast:ty), +) => {
        #[allow(dead_code)]
        pub fn $name(&self, uniform_name: &CStr, $($v: $t), +) -> Result<(), Error> {
            // try to get the location of the uniform
            let location = unsafe { gl::GetUniformLocation(self.program_id, uniform_name.to_bytes_with_nul().as_ptr() as *const GLchar) };
            // if the location is -1 the uniform was not found
            if location == -1 {
                return Err(Error::SetUniformError {
                    name: uniform_name.to_string_lossy().into_owned(),
                    message: "Uniform not found".into(),
                });
            }
            // set the uniform value with the correct cast
            unsafe { $gl_function(location, $($v as $cast), +) };
            Ok(())
        }
    };
}

impl Program {
    /// Create a new program from a list of shaders [`Shader`]
    /// returns an error if the program could not be created
    /// with kind of [`Error::LinkError`]
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Program, Error> {
        // create new program
        let id= unsafe {gl::CreateProgram()};

        // if the program could not be created return an error
        if id == 0 {
            return Err(Error::LinkError {
                name: "Unknown".into(),
                message: "Failed to create program".into(),
            });
        }

        // attach all shaders from the list to the program
        for shader in shaders {
            unsafe {gl::AttachShader(id, shader.id()) };
        }

        // link the program
        unsafe{ gl::LinkProgram(id) };

        // fetch link status
        let mut status = 0;
        unsafe{ gl::GetProgramiv(id, gl::LINK_STATUS, &mut status)};

        // if the link failed return an error
        if status == 0 {
            // get the error message from the program and return error
            let mut len = 0;
            unsafe{ gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) } ;

            let mut buffer = vec![0u8; len as usize];
            unsafe{ gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar)};

            return Err(Error::LinkError {
                name: "Unknown".into(),
                message: String::from_utf8(buffer).unwrap(),
            });
        }

        // detach all shaders from the program
        for shader in shaders {
            unsafe{ gl::DetachShader(id, shader.id())};
        }

        Ok(Program {
            program_id: id,
        })
    }

    /// wrapper around [`gl::UseProgram()`]
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.program_id); }
    }

    /// get the program id
    #[allow(dead_code)]
    pub fn id(&self) -> GLuint {
        self.program_id
    }

    // generate the set_uniform functions
    set_shader_uniform!(set_f32, gl::Uniform1f, x: f32);
    set_shader_uniform!(set_i32, gl::Uniform1i, x: i32);
    set_shader_uniform!(set_f32_2, gl::Uniform2f, x: f32, y: f32);
    set_shader_uniform!(set_f32_3, gl::Uniform3f, x: f32, y: f32, z: f32);
    set_shader_uniform!(set_bool, gl::Uniform1i, x: bool, i32);
}

impl Drop for Program {
    // delete the program when it goes out of scope
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program_id); }
    }
}