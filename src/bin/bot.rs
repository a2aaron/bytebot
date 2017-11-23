extern crate bytebeat;
extern crate dotenv;
extern crate egg_mode;
extern crate tokio_core;

use std::io::Read;
use std::env;

use egg_mode::{media, Token, KeyPair};
use egg_mode::tweet::DraftTweet;

mod util;


fn main() {
    dotenv::dotenv().unwrap();
    let auth = get_auth().unwrap();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let beat = "t t >> "; // 255 % & t 13 >> t & -";
    let code = bytebeat::parse_beat(beat).unwrap();

    println!("Encoding video...");
    util::generate_video(&code, "out.mp4");
    let data = {
        let mut data = Vec::new();
        std::fs::File::open("out.mp4")
            .unwrap()
            .read_to_end(&mut data)
            .unwrap();
        data
    };
    println!("Encoded!");

    println!("Uploading video to twitter...");
    let media_handle = core.run(
        media::UploadBuilder::new(&data[..], media::media_types::video_mp4())
            .category(media::MediaCategory::Video)
            .call(&auth, &handle),
    ).unwrap();
    println!("Uploaded!");

    println!("Posting tweet...");
    core.run(
        DraftTweet::new(beat)
            .media_ids(&[media_handle.media_id][..])
            .send(&auth, &handle),
    ).unwrap();
    println!("Posted!");
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
