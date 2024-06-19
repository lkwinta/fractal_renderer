use std::ffi::CString;
use gl::types::{GLchar, GLenum, GLuint};
use crate::resources::Resources;
use crate::renderer::Error;

/// Wrapper around an OpenGL shader
/// hides all unsafe calls and provides a simple interface to compile the shader
/// also includes a drop implementation to delete the shader when it goes out of scope
pub struct Shader {
    shader_id: GLuint,
}

impl Shader{
    /// Create a new shader from a resource file, returns an [`Error`] if the shader failed to compile or load
    #[allow(dead_code)]
    pub fn from_resources(res: &Resources, res_name: &str, shader_type: u32) -> Result<Shader, Error> {
        // Load the shader source from the resource file
        let shader_resource = res.load_string(res_name).map_err(|e| Error::ResourceLoad {
            name: res_name.into(),
            inner: e,
        });

        // If the resource failed to load, return the error
        if shader_resource.is_err() {
            return Err(shader_resource.unwrap_err());
        }

        let source = CString::new(shader_resource.unwrap()).unwrap();

        Self::compile_shader(&source, shader_type).map(|shader_id| Shader { shader_id } )
    }

    /// Create a new shader from a source [`string`](CString), returns an [`Error`] if the shader failed to compile
    #[allow(dead_code)]
    pub fn from_source(source: &CString, shader_type: u32) -> Result<Shader, Error> {
        Self::compile_shader(source, shader_type).map(|shader_id| Shader { shader_id })
    }

    /// Compile the shader from a source [`string`](CString), returns an [`Error::CompileError`] if the shader failed to compile
    fn compile_shader(source: &CString, shader_type: GLenum) -> Result<GLuint, Error> {
        // Create a new shader
        let shader = unsafe {gl::CreateShader(shader_type)};

        // If the shader failed to create, return an error
        if shader == 0 {
            return Err(Error::CompileError {
                name: "Unknown".into(),
                message: "Failed to create shader".into(),
            });
        }

        // Try to attach shader source to shader object and compile it
        unsafe {
            gl::ShaderSource(shader, 1, &source.as_c_str().as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }

        // Check if the shader compiled successfully
        let mut success= 0;
        unsafe {gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success)};

        // If the shader failed to compile, return an error
        if success == 0 {
            // get the error message from the shader and return error
            let mut len = 0;
            unsafe {gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len)};

            let mut buffer = vec![0; len as usize];
            unsafe { gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar) };

            let message = String::from_utf8(buffer).unwrap();
            return Err(Error::CompileError {
                name: "Unknown".into(),
                message,
            });
        }

        Ok(shader)
    }

    /// Get the shader id
    pub fn id(&self) -> GLuint {
        self.shader_id
    }
}

impl Drop for Shader {
    /// Drop the shader and delete it from the GPU if it goes out of scope
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.shader_id); }
    }
}