use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum _Err
{
    FileError(io::Error),
    NoSuchDir(PathBuf),
    InvalidPixel(u8)
}

impl From<io::Error> for _Err
{
    fn from(err: io::Error) -> _Err
    { // FileError is a better name for it since it can be raised when writing, reading or doing other actions
        _Err::FileError(err)
    }
}

#[allow(dead_code)] // get rid of warnings
impl _Err
{
    pub fn invalid_pixel(pixel: u8) -> _Err
    {
        _Err::InvalidPixel(pixel)
    }
    pub fn no_such_dir(dir: PathBuf) -> _Err
    {
        _Err::NoSuchDir(dir)
    }
}