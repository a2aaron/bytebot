extern crate bytebeat;

use std::io::{Read, Write};

#[derive(Copy, Clone)]
struct Color([u8; 3]);


const WIDTH: usize = 512;
const HEIGHT: usize = 256;
const SIZE: usize = WIDTH * HEIGHT * 2;
const HZ: usize = 8000;
const FPS: usize = 30;
const FRAME_COUNT: usize = SIZE * FPS / HZ;


fn main() {
    // Read in the expression
    let code = {
        println!("Enter a bytebeat command and then press Ctrl-D:");
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).unwrap();
        match bytebeat::parse_beat(&text) {
            Ok(code) => code,
            Err(_) => {
                eprintln!("Error parsing bytebeat");
                std::process::exit(1);
            }
        }
    };
    println!("Encoding...");

    // Generate the audio data
    let data = {
        let mut data = [0; SIZE];
        for i in 0..SIZE {
            data[i] = bytebeat::eval_beat(&code, i as f64).unwrap() as u8;
        }
        data
    };

    // Write the raw audio file as PCM
    {
        let mut audio_file = std::fs::File::create("audio").unwrap();
        audio_file.write_all(&data).unwrap();
    }

    // Write the raw images as PPM
    {
        let video_file = std::fs::File::create("video").unwrap();
        let mut image = [Color([0, 0, 0]); WIDTH * HEIGHT];
        let mut out = std::io::BufWriter::new(video_file);

        for frame in 0..FRAME_COUNT {
            write_frame(&mut out, &mut image, &data, frame, Color([255, 100, 16]));
        }
    }

    // Use FFmpeg to convert the raw content into a video
    let fps = format!("{}", FPS);
    let hz = format!("{}", HZ);
    std::process::Command::new("ffmpeg")
        .args(&["-r", &fps, "-f", "image2pipe", "-i", "video"])
        .args(&["-f", "u8", "-ar", &hz, "-ac", "1", "-i", "audio"])
        .args(&["-pix_fmt", "yuv420p", "-y", "-s", "640x360", "out.mp4"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::fs::remove_file("audio").unwrap();
    std::fs::remove_file("video").unwrap();
}

fn write_ppm<W: Write>(out: &mut W, buf: &[Color], width: usize, height: usize) {
    write!(out, "P6\n{} {}\n255\n", width, height).unwrap();
    for pix in buf {
        out.write(&pix.0[..]).unwrap();
    }
}

fn write_frame<W: Write>(
    out: &mut W,
    image: &mut [Color],
    data: &[u8],
    frame: usize,
    color: Color,
) {
    // Compute the offsets
    let (col, off) = if frame < FRAME_COUNT / 4 {
        (WIDTH * frame * 2 / FRAME_COUNT, 0)
    } else if frame < 3 * FRAME_COUNT / 4 {
        (
            WIDTH / 2,
            (frame - FRAME_COUNT / 4) * WIDTH * 2 / FRAME_COUNT,
        )
    } else {
        (WIDTH * (frame - FRAME_COUNT / 2) * 2 / FRAME_COUNT, WIDTH)
    };

    // Draw the background
    for i in 0..WIDTH * HEIGHT {
        let idx = (i % HEIGHT) * WIDTH + (i / HEIGHT);
        let x = data[i + off * HEIGHT];
        image[idx] = color * x;
    }

    // Draw the cursor
    for row in 0..HEIGHT {
        image[row * WIDTH + col] = Color([255, 255, 255]);
    }

    // Output an image
    write_ppm(out, &image, WIDTH, HEIGHT);
}

impl std::ops::Mul<u8> for Color {
    type Output = Color;
    fn mul(self, rhs: u8) -> Color {
        let Color(lhs) = self;
        let r = lhs[0] as u16 * rhs as u16;
        let g = lhs[1] as u16 * rhs as u16;
        let b = lhs[2] as u16 * rhs as u16;
        Color([(r >> 8) as u8, (g >> 8) as u8, (b >> 8) as u8])
    }
}
