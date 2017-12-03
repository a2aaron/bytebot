#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate bytebeat;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 2 {
        let time = data[0] as i64;
        if let Ok(s) = std::str::from_utf8(&data[1..]) {
            if let Ok(program) = bytebeat::parse_beat(s) {
                if let Ok(program) = bytebeat::compile(program) {
                    let _ = bytebeat::eval_beat(&program, time);
                }
            }
        }
    }
});
