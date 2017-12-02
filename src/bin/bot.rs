extern crate bytebeat;
extern crate dotenv;
extern crate egg_mode;
extern crate tokio_core;

use std::io::{self, Read};
use std::env;

use bytebeat::Program;
use dotenv::dotenv;
use egg_mode::{media, Token, KeyPair};
use egg_mode::tweet::DraftTweet;
use tokio_core::reactor::Core;

mod util;


fn main() {
    dotenv().ok();
    let auth = get_auth().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    println!("Generating beat...");
    let (code, text) = generate_beat();
    println!("Generated!");
    println!("Using formula: {}", text);

    println!("Rendering video...");
    let video_data = encode_video(&code).unwrap();
    println!("Rendered!");

    println!("Uploading video to twitter...");
    let upload = media::UploadBuilder::new(&video_data[..], media::media_types::video_mp4())
        .category(media::MediaCategory::Video);
    let media_handle = core.run(upload.call(&auth, &handle)).unwrap();
    println!("Uploaded!");

    println!("Posting tweet...");
    core.run(
        DraftTweet::new(text)
            .media_ids(&[media_handle.media_id][..])
            .send(&auth, &handle),
    ).unwrap();
    println!("Posted!");
}

fn generate_beat() -> (Program, String) {
    println!("Reading a beat from stdin:");
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let cmds = bytebeat::parse_beat(&buf).unwrap();
    (bytebeat::compile(cmds).unwrap(), buf)
}

fn encode_video(code: &Program) -> io::Result<Vec<u8>> {
    util::generate_video(&code, "out.mp4");
    let mut data = Vec::new();
    std::fs::File::open("out.mp4")?.read_to_end(&mut data)?;
    Ok(data)
}

fn get_auth() -> Result<Token, std::env::VarError> {
    let consumer = KeyPair::new(
        env::var("BYTEBEAT_CONSUMER_KEY")?,
        env::var("BYTEBEAT_CONSUMER_SECRET")?,
    );
    let access = KeyPair::new(
        env::var("BYTEBEAT_ACCESS_KEY")?,
        env::var("BYTEBEAT_ACCESS_SECRET")?,
    );
    Ok(Token::Access { consumer, access })
}
