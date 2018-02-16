extern crate bytebot_rpn as rpn;

use std::io::{Read, Write};

fn main() {
    let mut code = String::new();
    std::io::stdin().read_to_string(&mut code).unwrap();
    let code = rpn::parse_beat(&code).unwrap();
    let code = rpn::compile(code).unwrap();
    for i in 0..60 {
        let buf: Vec<_> = (i * 8000..(i + 1) * 8000)
            .map(|t| rpn::eval_beat(&code, t).into())
            .collect();
        std::io::stdout().write_all(&buf[..]).unwrap();
    }
}
