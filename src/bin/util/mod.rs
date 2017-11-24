extern crate bytebeat;

use bytebeat::Program;
use bytebeat::encode::Color;

const WIDTH: usize = 512;
const HEIGHT: usize = 256;
const FPS: usize = 30;

pub fn generate_video(code: &Program, fname: &str) {
    let hz = code.hz().unwrap_or(8000) as usize;
    let frame_count = FPS * 30;
    let size = frame_count * hz / FPS;
    let full_width = size / HEIGHT;

    // Generate the audio data
    let data = {
        let mut data = vec![0; size];
        for i in 0..size {
            data[i] = bytebeat::eval_beat(&code, i as f64).unwrap().into();
        }
        data
    };

    let mut encoder = bytebeat::encode::EncoderConfig::with_dimensions(WIDTH, HEIGHT)
        .fps(FPS)
        .audio_rate(hz)
        .audio_path("audio.pcm")
        .video_path("video.ppm")
        .build()
        .unwrap();

    encoder.write_audio(&data).unwrap();

    let mut image = [Color([0, 0, 0]); WIDTH * HEIGHT];
    for frame in 0..frame_count {
        let x = frame * full_width / frame_count;
        let bg_offset = x.saturating_sub(WIDTH / 2).min(full_width - WIDTH);
        let cursor_col = x - bg_offset;

        render_frame(
            &mut image,
            &data,
            bg_offset,
            cursor_col,
            code.bg().unwrap_or(Color([255, 100, 16])),
            code.fg().unwrap_or(Color([64, 128, 255])),
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
    bg_offset: usize,
    cursor_col: usize,
    bg_color: Color,
    wave_color: Color,
    scan_color: Color,
) {
    // Draw the background
    for i in 0..WIDTH * HEIGHT {
        let idx = (i % HEIGHT) * WIDTH + (i / HEIGHT);
        let x = data[i + bg_offset * HEIGHT];
        image[idx] = bg_color * x;
    }

    // Draw the waveform
    let waveform_col = bg_offset + cursor_col;
    for row in 0..HEIGHT - 1 {
        let x0 = data[waveform_col * HEIGHT + row];
        let x1 = data[waveform_col * HEIGHT + row + 1];
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
        image[row * WIDTH + cursor_col] = scan_color;
    }
}
