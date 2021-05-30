use super::decode_pixel; // use the super keyword to get other files.
use self::decode_pixel::DecodePixel; // repl is weird, so you have to put self before the filename.
use self::decode_pixel::DecodePixelFuncs;
use super::errors;
use self::errors::_Err;

use std::env; // get current_dir
use std::path::PathBuf; // working with paths
use std::fs;
use std::io::Read;
use std::fs::File;

impl DecodePixelFuncs for DecodePixel {
    fn new_decoder(filename: String) -> Result<DecodePixel, _Err> {
        let mut DI = DecodePixel {
            dir: PathBuf::new(), // init a empty PathBuf struct
            content: Vec::new(), // create new vector
            pixel_array: Vec::new(),
            size: 0,
            index: 0
        };

        let mut _dir: PathBuf = PathBuf::from(env::current_dir()?);
        _dir = _dir.join(filename);

        if _dir.exists() {
            DI.dir = _dir;

            // read file content
            let mut f = File::open(&DI.dir).expect("No such file");

            // Read the metadata(since it's not a UTF-8 file.)
            let meta = fs::metadata(DI.dir.clone()).expect("cannot read metadata");

            // Read it into a buffer
            let mut buffer = vec![0; meta.len() as usize];
            f.read(&mut buffer).expect("Couldn't read");

            // Getting the size
            DI.size = fs::metadata(DI.dir.clone()).unwrap().len() as usize;

            DI.content = buffer;

            return Ok(DI.clone());
        }
        Err(_Err::no_such_dir(_dir))
    }

    fn begin_decode(&mut self) -> Result<DecodePixel, _Err>
    {
        // Check beginning of file(should be .PNG)
        let mut file_beginning: Vec<char> = Vec::new();
        for i in 0..8 {
            file_beginning.push(self.content[i] as char);
        }

        if file_beginning[0] == 0x89 as char {
            let val: String = file_beginning[1..4].into_iter().collect();

            self.index = 4;

            /*
             * Beginning of file must consist of:
             *  -> [89 50 4e 47], [0d 0a 1a 0a].
             *
             * We check to see if [, ...] = PNG([50, 42, 47], ignore first value). If it is, check the next 4 bytes.
             */
            if val == "PNG" {
                // Checking for [0d 0a 1a 0a]
                if file_beginning[self.index   ] == 0x0d as char
                && file_beginning[self.index + 1] == 0x0a as char
                && file_beginning[self.index + 2] == 0x1a as char
                && file_beginning[self.index + 3] == 0x0a as char
                {
                    
                }
            }
        }

        Ok(self.clone())
    }
}