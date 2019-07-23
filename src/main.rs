extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    let mut output = "output.txt".to_string();
    let mut input = "input.txt".to_string();
    let mut d_temp: String = "".to_string();
    let mut dimensions = vec![0; 2];
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Converts an image to ascii art.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Increases verbosity");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Input image to convert")
            .required();
        ap.refer(&mut output)
            .add_option(&["-o", "--output"], Store,
            "Output file location (Defaults to output.txt)");
        ap.refer(&mut d_temp)
            .add_option(&["-d", "--dimensions"], Store,
            "(optional) Resize image for ascii output dimensions [Example: -d 25x25]");
        ap.parse_args_or_exit();
    }

    if !d_temp.is_empty() {
        dimensions = d_temp.split('x').map(
            |s| s.parse().unwrap()
        ).collect();
    }

    if verbose {
        println!("add verbosity thing later lol");
    }
    println!("it didnt crash");
    println!("{}\n{:?}", input, dimensions);
}
