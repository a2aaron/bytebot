extern crate bytebeat;

use std::io::Read;

mod generate_video;

fn main() {
    // Read in the expression
    let code = {
        println!("Enter a bytebeat command and then press Ctrl-D:");
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).unwrap();
        match bytebeat::parse_beat(&text) {
            Ok(code) => code,
            Err(_) => {
                eprintln!("Error parsing bytebeat");
                std::process::exit(1);
            }
        }
    };
    println!("Encoding...");
    generate_video::generate_video(&code, "out.mp4");
}
