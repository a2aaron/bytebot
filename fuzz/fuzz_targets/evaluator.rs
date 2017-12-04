#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate bytebeat;

use bytebeat::Val;


fuzz_target!(|data: &[u8]| {
    if data.len() < 9 {
        return;
    }

    let time_kind = data[0];
    let raw_num = (data[1] as u64) | (data[2] as u64) << 08 | (data[3] as u64) << 16 |
        (data[4] as u64) << 24 | (data[5] as u64) << 32 |
        (data[6] as u64) << 40 | (data[7] as u64) << 48 | (data[8] as u64) << 56;
    let time = if time_kind == 0 {
        Val::I(raw_num as i64)
    } else {
        Val::F(f64::from_bits(raw_num))
    };

    if let Ok(s) = std::str::from_utf8(&data[9..]) {
        if let Ok(program) = bytebeat::parse_beat(s) {
            if let Ok(program) = bytebeat::compile(program) {
                let _ = bytebeat::eval_beat(&program, time);
            }
        }
    }
});
