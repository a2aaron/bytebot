extern crate bytebeat;

mod generate_video;

fn main() {
    let code = bytebeat::parse_beat("t t 255 % & t 13 >> t & -").unwrap();
    generate_video::generate_video(&code, "out.mp4");
}
