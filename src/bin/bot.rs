extern crate bytebeat;

mod util;

fn main() {
    let code = bytebeat::parse_beat("t t 255 % & t 13 >> t & -").unwrap();
    util::generate_video(&code, "out.mp4");
}
