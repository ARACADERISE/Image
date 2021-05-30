#[allow(non_snake_case)]
mod App;

use App::decode_pixel;
use self::decode_pixel::DecodePixel;
use self::decode_pixel::DecodePixelFuncs;

fn main()
{
    #![allow(non_snake_case)] // git rid of warnings
    match DecodePixel::new_decoder("images/image.png".to_string()) {
        Ok(mut t) => {
            match t.begin_decode() {
                Ok(T) => {
                    println!("{:?}", T);
                },
                Err(E) => panic!("{:?}", E)
            }
        },
        Err(e) => panic!("{:?}", e)
    }
}
