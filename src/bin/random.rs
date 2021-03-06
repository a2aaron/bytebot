extern crate bytebeat;
extern crate rand;

use rand::Rng;
use bytebeat::random;
mod util;

fn main() {
    let code = gen_beat(2, 7);
    println!("{}", code);
}

fn gen_beat(min: usize, max: usize) -> bytebeat::Program {
    loop {
        let length = rand::thread_rng().gen_range::<usize>(min, max);
        let code = random::random_t_multiply(length);
        let code = bytebeat::compile(code).unwrap();
        let buf: Vec<_> = (0..5 * 8000)
            .map(|t| bytebeat::eval_beat(&code, t))
            .collect();

        let is_silent = buf.iter().all(|&x| x == buf[0]);

        if is_silent {
            // Write to standard error because writing to stdout will
            // pipe that to sox/mplayer.
            eprintln!("Skipping: {}", code)
        } else {
            return code;
        }
    }
}
