extern crate argparse;
extern crate image;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::io::prelude::*;
use std::fs;
//use image::{save_buffer_with_format, ColorType};

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

    for i in 0..w {
        for j in 0..h {
            //let px = rgb.get_pixel(i as u32, j as u32);
            let offset = inc*4;
            let px_val: u16 = px[offset] as u16 + 
                               px[offset+1] as u16 + 
                               px[offset+2] as u16;
            let px_avg: u8 = if (px_val / 3) <= 255 {(px_val / 3) as u8} else {255};
            if verbose {
                println!("{}:{}:{},{},{}", offset+2, px_avg, px[offset], px[offset+1], px[offset+2]);
            }
            let val = match px_avg {
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
        let height = a_vec.len();
        let width = a_vec[0].len();
        for line in a_vec.iter() {
            for px in line.iter() {
                a_file.write(px).unwrap();
            }
        }
        
    }

    //img.save(output).ok();
    //println!("{:?}", ascii_img)
    save_txt(ascii_img, output);
}
