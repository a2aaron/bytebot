extern crate bytebot_rpn as rpn;

use std::io::Read;

mod util;

fn main() {
    // Read in the expression
    let code = {
        println!("Enter a bytebeat command and then press Ctrl-D:");
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).unwrap();
        match rpn::parse_beat(&text) {
            Ok(code) => {
                match rpn::compile(code) {
                    Ok(code) => code,
                    Err(compile_error) => {
                        eprintln!("{}", compile_error);
                        std::process::exit(1);
                    }
                }
            }
            Err(parse_error) => {
                eprintln!("Error parsing bytebeat: {}", parse_error);
                std::process::exit(1);
            }
        }
    };
    println!("Encoding...");
    util::generate_video(&code, "out.mp4");
}
