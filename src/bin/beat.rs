extern crate bytebeat;
extern crate rand;
use std::io::{Read, Write};
use rand::Rng;

fn main() {
    // let mut code = String::new();
    // let code = "t -1 <<".to_string();
    // std::io::stdin().read_to_string(&mut code).unwrap();
    // let code = bytebeat::parse_beat(&code).unwrap();
    for j in 0..200 {
        let length = rand::thread_rng().gen_range::<usize>(1, 16);
        let code = bytebeat::random_beat(length);
        for i in 0..1 {
            let buf: Vec<_> = (i * 8000..(i + 1) * 8000)
                .map(|t| bytebeat::eval_beat(&code, t as f64).unwrap() as u8)
                .collect();
            let mut is_silent = true;
            for ele in &buf {
                if *ele != 0 {
                    is_silent = false;
                    break;
                }
            }
            if is_silent {
                eprintln!("Skipping: {}", bytebeat::format_beat(&code))
            } else {
                std::io::stdout().write_all(&buf[..]).unwrap();
            }
        }
        eprintln!("{}", bytebeat::format_beat(&code));
    }
}
