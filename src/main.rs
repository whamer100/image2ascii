extern crate argparse;
extern crate image;

use argparse::{ArgumentParser, StoreTrue, Store};
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
        println!("add verbosity thing later lol");
    }
    println!("it didnt crash");
    println!("{}\n{:?}", input, dimensions);
    println!("image time yay");

    let mut img = image::open(input).unwrap();
    if do_resize {
        img = img.resize_exact(dimensions[0], dimensions[1], image::imageops::FilterType::Lanczos3)
    }

    println!("if you made it this far then it read the image woah");

    img.save(output).ok();
}
