extern crate bytebeat;
extern crate rand;

use std::io::{Read, Write};
use rand::Rng;


fn main() {
    for _ in 0..200 {
        let length = rand::thread_rng().gen_range::<usize>(1, 16);
        let code = bytebeat::random::random_beat(length);
        let buf: Vec<_> = (0..8000)
            .map(|t| bytebeat::eval_beat(&code, t as f64).unwrap() as u8)
            .collect();

        let is_silent = buf.iter().all(|&x| x == buf[0]);

        if is_silent {
            // Write to standard error because writing to stdout will 
            // pipe that to sox/mplayer.
            eprintln!("Skipping: {}", bytebeat::format_beat(&code))
        } else {
            std::io::stdout().write_all(&buf[..]).unwrap();
        }
        eprintln!("{}", bytebeat::format_beat(&code));
    }
}
