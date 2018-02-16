extern crate bytebeat;

use bytebeat::Program;
use bytebeat::encode::{Color, EncoderConfig};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const BG_HEIGHT: usize = 256;
const WV_HEIGHT: usize = HEIGHT - BG_HEIGHT;
const FPS: usize = 15;

pub fn generate_video(code: &Program, fname: &str) {
    let hz = code.hz().unwrap_or(8000) as usize;
    let frame_count = FPS * 30;
    let size = frame_count * hz / FPS;
    let full_width = size / BG_HEIGHT;

    // Generate the audio data
    let data = {
        let mut data = vec![0; size];
        for i in 0..size {
            data[i] = bytebeat::eval_beat(&code, i as f64).into();
        }
        data
    };

    let mut encoder = EncoderConfig::with_dimensions(WIDTH, HEIGHT)
        .fps(FPS)
        .audio_rate(hz)
        .output_dimensions(640, 360)
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
    for row in 0..BG_HEIGHT {
        for col in 0..WIDTH {
            let i = row + col * BG_HEIGHT;
            let x = data[i + bg_offset * BG_HEIGHT];
            image[row * WIDTH + col] = bg_color * x;
        }
    }

    // Clear the waveform
    for row in BG_HEIGHT..HEIGHT {
        for col in 0..WIDTH {
            image[row * WIDTH + col] = Color([0, 0, 0]);
        }
    }

    // Draw the waveform
    let waveform_col = bg_offset + cursor_col;
    for sample in 0..BG_HEIGHT - 1 {
        let x0 = data[waveform_col * BG_HEIGHT + sample] as usize * WV_HEIGHT / 256;
        let x1 = data[waveform_col * BG_HEIGHT + sample + 1] as usize * WV_HEIGHT / 256;
        let mid = ((x0 + x1) / 2) as usize;
        let col = sample * WIDTH / BG_HEIGHT;

        for r in x0.min(mid)..x0.max(mid) + 1 {
            let row = HEIGHT - r - 1;
            image[row * WIDTH + col] = wave_color;
            image[row * WIDTH + col + 1] = wave_color;
        }
        for r in x1.min(mid)..x1.max(mid) + 1 {
            let row = HEIGHT - r - 1;
            image[row * WIDTH + col + 1] = wave_color;
            image[row * WIDTH + col + 2] = wave_color;
        }
    }

    // Draw the cursor
    for row in 0..BG_HEIGHT {
        if cursor_col > 0 {
            image[row * WIDTH + cursor_col] = scan_color;
        }
        image[row * WIDTH + cursor_col] = scan_color;
    }
}
