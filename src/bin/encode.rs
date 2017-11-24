extern crate bytebeat;

use std::io::Read;

mod util;

fn main() {
    // Read in the expression
    let code = {
        println!("Enter a bytebeat command and then press Ctrl-D:");
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).unwrap();
        match bytebeat::parse_beat(&text) {
            Ok(code) => {
                match bytebeat::compile(code) {
                    Ok(code) => code,
                    Err(_) => {
                        eprintln!("Error compiling bytebeat");
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => {
                eprintln!("Error parsing bytebeat");
                std::process::exit(1);
            }
        }
    };
    println!("Encoding...");
    util::generate_video(&code, "out.mp4");
}
