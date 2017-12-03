#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate bytebeat;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(program) = bytebeat::parse_beat(s) {
            if let Ok(program) = bytebeat::compile(program) {
                let _ = bytebeat::eval_beat(&program, 0.0);
            }
        }
    }
});
