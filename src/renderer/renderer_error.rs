use crate::resources;

/// Error type for the renderer module
#[derive(Debug)]
pub enum Error {
    ResourceLoad { name:String, inner: resources::Error},
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
    SetUniformError { name: String, message: String },
}
