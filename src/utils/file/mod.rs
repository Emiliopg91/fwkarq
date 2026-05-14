use std::{fs, io, path::Path};

pub struct FileUtils;

impl FileUtils {
    pub fn exists<T>(path: T) -> bool
    where
        T: AsRef<Path>,
    {
        path.as_ref().exists()
    }

    pub fn is_file<T>(path: T) -> bool
    where
        T: AsRef<Path>,
    {
        path.as_ref().is_file()
    }

    pub fn is_dir<T>(path: T) -> bool
    where
        T: AsRef<Path>,
    {
        path.as_ref().is_dir()
    }

    pub fn write<T, I>(path: T, content: I) -> io::Result<()>
    where
        T: AsRef<Path>,
        I: AsRef<[u8]>,
    {
        fs::write(path, content.as_ref())
    }

    pub fn read<T>(path: T) -> io::Result<String>
    where
        T: AsRef<Path>,
    {
        fs::read_to_string(path)
    }
}
