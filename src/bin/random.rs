extern crate bytebeat;
extern crate rand;

use rand::Rng;
use bytebeat::random;
mod util;

fn main() {
    for _ in 0..10 {
        let length = rand::thread_rng().gen_range::<usize>(2, 7);
        let code = random::random_t_multiply(length);
        println!("{}", bytebeat::format_beat(&code));
    }

    let length = rand::thread_rng().gen_range::<usize>(2, 7);
    let code = random::random_t_multiply(length);
    println!("{}", bytebeat::format_beat(&code));
    let code = bytebeat::compile(code).unwrap();
    // eprintln!("{}", bytebeat::format_beat(&code));
    let buf: Vec<_> = (0..5*8000)
        .map(|t| bytebeat::eval_beat(&code, t))
        .collect();

    let is_silent = buf.iter().all(|&x| x == buf[0]);

    if is_silent {
        // Write to standard error because writing to stdout will 
        // pipe that to sox/mplayer.
        eprintln!("Skipping: {}", code)
    } else {
        println!("Encoding... (formula: {})", code);
        util::generate_video(&code, "out.mp4");
        eprintln!("{}", code);
    }
}