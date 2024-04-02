use crate::resources;

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name:String, inner: resources::Error},
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
    SetUniformError { name: String, message: String },
}
