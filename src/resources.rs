use std::{fs, io};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = std::env::current_exe()
            .map_err(|_| Error::FailedToGetExePath )?;

        let exe_path = exe_file_name.parent().
            ok_or(Error::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_string(&self, resource_name: &str) -> Result<String, Error> {
        let full_path_buff = resource_name_to_path(&self.root_path, resource_name);
        let file = fs::File::open(&full_path_buff);

        if file.is_err(){
            return Err(Error::Io(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to open file {}", full_path_buff.to_str().unwrap()))));
        }

        let mut buffer = String::new();
        file?.read_to_string(&mut buffer)?;

        if buffer.contains('\0'){
            return Err(Error::FileContainsNil);
        }

        if buffer.is_empty(){
            return Err(Error::Io(io::Error::new(io::ErrorKind::Other, "File is empty")));
        }

        Ok(buffer)
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/"){
        path = path.join(part);
    }

    return path;
}