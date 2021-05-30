// ToDo: Add support far ARGB(bmp images support ARGB)

use super::errors;
use self::errors::_Err;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DecodePixel
{
    pub dir: PathBuf, // path to the file
    pub content: Vec<u8>, // binary values 0-255(u8)
    pub pixel_array: Vec<u8>, // rgb values are 0-255(u8)
    pub size: usize,
    pub index: usize
}

pub trait DecodePixelFuncs
{
    fn new_decoder(filename: String) -> Result<DecodePixel, _Err>; // generate new DecodePixel struct
    fn begin_decode(&mut self) -> Result<DecodePixel, _Err>; // Decode the .png image(getting the header and pixel array.)
}