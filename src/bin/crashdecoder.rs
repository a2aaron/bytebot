extern crate bytebot_rpn as rpn;
use std::io::Read;

use rpn::Val;

fn main() {
    let mut data = Vec::new();
    let _ = std::io::stdin().read_to_end(&mut data);

    let time_kind = data[0];
    let raw_num = (data[1] as u64) | (data[2] as u64) << 08 | (data[3] as u64) << 16
        | (data[4] as u64) << 24 | (data[5] as u64) << 32 | (data[6] as u64) << 40
        | (data[7] as u64) << 48 | (data[8] as u64) << 56;
    let time = if time_kind == 0 {
        Val::I(raw_num as i64)
    } else {
        Val::F(f64::from_bits(raw_num))
    };
    let program = std::str::from_utf8(&data[9..]).unwrap();
    println!("code: \"{}\" at time: {:?}", program, time);
    match rpn::parse_beat(program) {
        Ok(program) => {
            println!("parsed: {:?}", program);
            match rpn::compile(program) {
                Ok(program) => {
                    println!("compiled: {:?}", program);
                    let value = rpn::eval_beat(&program, time);
                    println!("evaluted: {:?}", value);
                }
                Err(err) => println!("Could not compile: {}", err),
            }
        }
        Err(err) => println!("Could not parse: {}", err),
    }
}
