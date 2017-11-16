extern crate bytebeat;

use std::io::Write;

#[derive(Copy, Clone)]
struct Color([u8; 3]);



fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 256;
    const SIZE: usize = WIDTH * HEIGHT;
    const HZ: usize = 8000;
    const FPS: usize = 30;
    const FRAME_COUNT: usize = SIZE * FPS / HZ;

    let code = "t t 12 >> t 8 >> | 63 t 4 >> & & *";
    let code = bytebeat::parse_beat(code).unwrap();
    let data = {
        let mut data = [0; SIZE];
        for i in 0..SIZE {
            data[i] = bytebeat::eval_beat(&code, i as f64).unwrap() as u8;
        }
        data
    };

    {
        let mut audio_file = std::fs::File::create("audio").unwrap();
        audio_file.write_all(&data).unwrap();
    }

    {
        let video_file = std::fs::File::create("video").unwrap();
        let mut image = [Color([0, 0, 0]); WIDTH * HEIGHT];
        let mut out = std::io::BufWriter::new(video_file);

        for frame in 0..FRAME_COUNT {
            for i in 0..SIZE {
                let idx = (i % HEIGHT) * WIDTH + (i / HEIGHT);
                image[idx] = Color([data[i], data[i] / 2, 0]);
            }

            let col = WIDTH * frame / FRAME_COUNT;
            for row in 0..HEIGHT {
                image[row * WIDTH + col] = Color([255, 255, 255]);
            }
            write_ppm(&mut out, &image, WIDTH, HEIGHT);
        }
    }

    let _ = std::fs::remove_file("out.mp4");
    let fps = format!("{}", FPS);
    let hz = format!("{}", HZ);
    std::process::Command::new("ffmpeg")
        .args(&["-r", &fps, "-f", "image2pipe", "-i", "video"])
        .args(&["-f", "u8", "-ar", &hz, "-ac", "1", "-i", "audio"])
        .args(&["-pix_fmt", "yuv420p", "out.mp4"])
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
