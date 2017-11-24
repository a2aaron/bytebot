extern crate bytebeat;

use bytebeat::Program;
use bytebeat::encode::Color;

const WIDTH: usize = 512;
const HEIGHT: usize = 256;
const SIZE: usize = WIDTH * HEIGHT * 2;
const HZ: usize = 8000;
const FPS: usize = 30;
const FRAME_COUNT: usize = SIZE * FPS / HZ;

pub fn generate_video(code: &Program, fname: &str) {
    // Generate the audio data
    let data = {
        let mut data = [0; SIZE];
        for i in 0..SIZE {
            data[i] = bytebeat::eval_beat(&code, i as f64).unwrap() as u8;
        }
        data
    };

    let mut encoder = bytebeat::encode::EncoderConfig::with_dimensions(WIDTH, HEIGHT)
        .fps(FPS)
        .audio_rate(HZ)
        .audio_path("audio.pcm")
        .video_path("video.ppm")
        .build()
        .unwrap();

    encoder.write_audio(&data).unwrap();

    let mut image = [Color([0, 0, 0]); WIDTH * HEIGHT];
    for frame in 0..FRAME_COUNT {
        render_frame(
            &mut image,
            &data,
            frame,
            Color([255, 100, 16]),
            Color([64, 128, 255]),
            Color([255, 255, 255]),
        );
        // Output a frame
        encoder.write_frame(&image).unwrap();
    }

    encoder.start_encode(fname).unwrap().wait().unwrap();
    encoder.remove_temp_files().unwrap();
}

fn render_frame(
    image: &mut [Color],
    data: &[u8],
    frame: usize,
    bg_color: Color,
    wave_color: Color,
    scan_color: Color,
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
        image[idx] = bg_color * x;
    }

    // Draw the waveform
    let x = WIDTH * frame * 2 / FRAME_COUNT;
    for row in 0..HEIGHT - 1 {
        let x0 = data[x * HEIGHT + row];
        let x1 = data[x * HEIGHT + row + 1];
        let col = row * WIDTH / HEIGHT;
        let (bot, top) = (x0.min(x1) as usize, x0.max(x1) as usize);
        let mid = (bot + top) / 2;
        for r in bot..mid + 1 {
            image[(255 - r) * WIDTH + col] = wave_color;
        }
        for r in mid..top + 1 {
            image[(255 - r) * WIDTH + col + 1] = wave_color;
        }
    }

    // Draw the cursor
    for row in 0..HEIGHT {
        image[row * WIDTH + col] = scan_color;
    }
}
