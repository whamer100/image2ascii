extern crate argparse;
extern crate image;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::io::prelude::*;
use std::fs;
//use image::{save_buffer_with_format, ColorType};

#[allow(non_snake_case)] // becuase this gets annoying to me

fn main() {
    let mut verbose = false;
    let mut output = "output.txt".to_string();
    let mut input = "input.txt".to_string();
    let mut do_resize: bool = false;
    let mut d_temp: String = "".to_string();
    let mut dimensions: Vec<u32> = vec![0; 2];
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Converts an image to ascii art.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Increases verbosity");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Input image to convert (Currently only supporting PNG)")
            .required();
        ap.refer(&mut output)
            .add_option(&["-o", "--output"], Store,
            "Output file location (Defaults to output.txt)");
        ap.refer(&mut d_temp)
            .add_option(&["-d", "--dimensions"], Store,
            "(optional) Resize image for ascii output dimensions [Example: -d 25x25]");
        ap.parse_args_or_exit();
    }

    fn d_check(r: Result<u32, std::num::ParseIntError>) -> Result<u32, std::num::ParseIntError> {
        if r.is_ok() {
            return r;
        }
        else {
            println!("Dimensions provided are not in the correct format.");
            std::process::exit(1);
        }
    }

    if !d_temp.is_empty() {
        dimensions = d_temp.split('x').map(
            |s| d_check(s.parse::<u32>()).unwrap()
        ).collect();

        if dimensions.len() != 2 {
            println!("Dimensions provided are not in the correct format.");
            std::process::exit(1);
        }

        do_resize = true;
    }

    if verbose {
        println!("it didnt crash");
        println!("{}\n{:?}", input, dimensions);
        println!("image time yay");
    }


    let mut img = image::open(input).unwrap();
    if do_resize {
        img = img.resize_exact(dimensions[0], dimensions[1], image::imageops::FilterType::Lanczos3)
    }
    
    if verbose {
        println!("if you made it this far then it read the image woah");
    }

    let _i = img.as_rgba8();
    let rgb: &image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;
    match _i {
        None => {
            println!("i broke lol");
            std::process::exit(1);
            },
        Some(x) => {
            //println!("{:?}", x);
            rgb = x;
            }
    }

    let w = rgb.width() as usize;
    let h = rgb.height() as usize;
    let mut ascii_img = vec![vec![0u8; w]; h];

    let px = rgb.to_vec();
    let mut inc: usize = 0;

    if verbose {
        println!("{}", px.len());
        println!("{}, {}, {}, {}", px[0], px[1], px[2], px[3]);
        println!("{}, {}, {}, {}", px[396], px[397], px[398], px[399]);
    }

    fn sRGBtoLin(c: f64) -> f64 {
        if c <= 0.04045_f64 {
            return c / 12.92_f64;
        } else {
            let mut cp: f64 = c + 0.055_f64;
            cp /= 1.055;
            return cp.powf(2.4_f64)
        }
    }

    fn YtoLstar(Y: f64) -> f64 {
        let m: f64 = 216_f64/24389_f64;
        let t0: f64 = 24389_f64/27_f64;
        if Y <= m {
            return Y * t0;
        } else {
            let Yr3 = Y.powf(1_f64/3_f64);
            return Yr3 * 116_f64 - 13_f64;
        }
    }

    fn rgb2lum(r: u8, g: u8, b: u8) -> u8 {
        let vR = r as f64 / 255_f64; // convert all integer values to decimal 0.0 ~ 1.0
        let vG = g as f64 / 255_f64;
        let vB = b as f64 / 255_f64;

        let cR = 0.2126_f64; // sRGB coefficients
        let cG = 0.7152_f64;
        let cB = 0.0722_f64;

        let Y = cR * sRGBtoLin(vR) + // luminance
                cG * sRGBtoLin(vG) +
                cB * sRGBtoLin(vB);

        let Lstar = YtoLstar(Y); // perceived luminance

        let mut Lstar_u8 = (Lstar * 2.55_f64) as u8; // convert to byte

        if Lstar_u8 >= 255 {
            Lstar_u8 = 255;
        }

        return Lstar_u8;
    }

    for j in 0..h {
        for i in 0..w {
            //let px = rgb.get_pixel(i as u32, j as u32);
            let offset = inc*4;
            let px_val = rgb2lum(px[offset], px[offset+1], px[offset+3]);
            if verbose {
                println!("{}:{}:{},{},{}", offset+2, px_val, px[offset], px[offset+1], px[offset+2]);
            }
            let val = match px_val {
                0  ...17  => '$',
                18 ...34  => '@',
                35 ...51  => '8',
                52 ...68  => '%',
                69 ...85  => '#',
                86 ...102 => '*',
                103...119 => '!',
                120...136 => '+',
                137...153 => '=',
                154...170 => '-',
                171...187 => ';',
                188...204 => ':',
                205...221 => ',',
                222...237 => '.',
                238...255 => ' ',
            };
            inc += 1;
            // encode data to u8 on the fly because im dumb lol
            let mut t = [0; 1];
            val.encode_utf8(&mut t);
            ascii_img[j][i] = t[0];
        }
    }

    fn save_txt(a_vec: std::vec::Vec<std::vec::Vec<u8>>, output: String) {
        let mut a_file = fs::File::create(output).unwrap();
        for line in a_vec.iter() {
            a_file.write_all(&line).unwrap();
            a_file.write(b"\n").unwrap();
        }
        
    }

    //img.save(output).ok();
    //println!("{:?}", ascii_img)
    save_txt(ascii_img, output);
}
