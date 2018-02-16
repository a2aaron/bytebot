use std::fs::{self, File};
use std::process;
use std::path::PathBuf;
use std::ops::Mul;
use std::io::{self, BufWriter, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color(pub [u8; 3]);

pub fn write_ppm<W: Write>(
    out: &mut W,
    buf: &[Color],
    width: usize,
    height: usize,
) -> io::Result<()> {
    write!(out, "P6\n{} {}\n255\n", width, height)?;
    for pix in buf {
        out.write(&pix.0[..])?;
    }
    Ok(())
}

impl Mul<u8> for Color {
    type Output = Color;
    fn mul(self, rhs: u8) -> Color {
        let Color(lhs) = self;
        let r = lhs[0] as u16 * rhs as u16;
        let g = lhs[1] as u16 * rhs as u16;
        let b = lhs[2] as u16 * rhs as u16;
        Color([(r >> 8) as u8, (g >> 8) as u8, (b >> 8) as u8])
    }
}

#[derive(Debug)]
pub struct EncoderConfig {
    width: usize,
    height: usize,
    fps: usize,
    out_dim: Option<(usize, usize)>,
    audio_rate: Option<usize>,
    audio_path: Option<PathBuf>,
    video_path: Option<PathBuf>,
}

impl EncoderConfig {
    pub fn with_dimensions(width: usize, height: usize) -> EncoderConfig {
        EncoderConfig {
            width,
            height,
            fps: 30,
            out_dim: None,
            audio_rate: None,
            audio_path: None,
            video_path: None,
        }
    }

    pub fn fps(self, fps: usize) -> EncoderConfig {
        EncoderConfig { fps, ..self }
    }

    pub fn audio_rate(self, rate: usize) -> EncoderConfig {
        EncoderConfig {
            audio_rate: Some(rate),
            ..self
        }
    }

    pub fn output_dimensions(self, width: usize, height: usize) -> EncoderConfig {
        EncoderConfig {
            out_dim: Some((width, height)),
            ..self
        }
    }

    pub fn audio_path<P: Into<PathBuf>>(self, path: P) -> EncoderConfig {
        EncoderConfig {
            audio_path: Some(path.into()),
            ..self
        }
    }

    pub fn video_path<P: Into<PathBuf>>(self, path: P) -> EncoderConfig {
        EncoderConfig {
            video_path: Some(path.into()),
            ..self
        }
    }

    pub fn build(self) -> Result<Encoder, &'static str> {
        // @Usability: Generate tempfiles if they're not specified
        let audio_path = self.audio_path.ok_or("audio path not set")?;
        let video_path = self.video_path.ok_or("video path not set")?;

        let audio_file =
            BufWriter::new(File::create(&audio_path).map_err(|_| "could not create audio file")?);
        let video_file =
            BufWriter::new(File::create(&video_path).map_err(|_| "could not create video file")?);

        let (out_width, out_height) = self.out_dim.unwrap_or((self.width, self.height));

        Ok(Encoder {
            width: self.width,
            height: self.height,
            fps: self.fps,
            audio_rate: self.audio_rate.ok_or("audio rate not set")?,
            out_width,
            out_height,
            audio_path,
            video_path,
            audio_file,
            video_file,
        })
    }
}

#[derive(Debug)]
pub struct Encoder {
    width: usize,
    height: usize,
    fps: usize,
    out_width: usize,
    out_height: usize,
    audio_rate: usize,
    audio_path: PathBuf,
    video_path: PathBuf,
    audio_file: BufWriter<File>,
    video_file: BufWriter<File>,
}

impl Encoder {
    pub fn write_audio(&mut self, audio: &[u8]) -> io::Result<()> {
        self.audio_file.write_all(&audio)
    }

    pub fn write_frame(&mut self, color: &[Color]) -> io::Result<()> {
        write_ppm(&mut self.video_file, &color, self.width, self.height)
    }

    pub fn start_encode(&mut self, out: &str) -> Result<process::Child, io::Error> {
        let fps = format!("{}", self.fps);
        let hz = format!("{}", self.audio_rate);
        let size = format!("{}x{}", self.out_width, self.out_height);

        // Flush the file buffers so FFmpeg can see them all
        self.audio_file.flush()?;
        self.video_file.flush()?;

        // Use FFmpeg to convert the raw content into a video
        process::Command::new("ffmpeg")
            .args(&["-r", &fps, "-f", "image2pipe", "-i"])
            .arg(&self.video_path)
            .args(&["-f", "u8", "-ar", &hz, "-ac", "1", "-i"])
            .arg(&self.audio_path)
            .args(&["-pix_fmt", "yuv420p", "-y", "-movflags", "+faststart"])
            .args(&["-c:a", "aac", "-strict", "-2", "-b:a", "96k", "-ar", "16k"])
            .args(&["-c:v", "libx264", "-b:v", "768K"])
            .args(&["-s", &size])
        // "-sws_flags", "neighbor"
            .arg(out)
            .spawn()
    }

    pub fn remove_temp_files(self) -> io::Result<()> {
        fs::remove_file(self.audio_path)?;
        fs::remove_file(self.video_path)?;
        Ok(())
    }
}
