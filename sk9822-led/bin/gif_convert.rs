use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, GenericImageView};
use matrix_animation::Animation;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const LED_WIDTH: usize = 14;
const LED_HEIGHT: usize = 14;
const BYTES_PER_LED: usize = 4;
const GIF_DIR: &str = "sk9822-led/gifs/";
const BIN_DIR: &str = "sk9822-led/animations/";
const BIN_EXTENSION: &str = "bin";

fn gif_to_animation(
    gif_path: PathBuf,
    output_path: PathBuf,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!(
        "Converting GIF: {} -> {}",
        gif_path.display(),
        output_path.display()
    );
    let file = File::open(gif_path)?;
    let decoder = GifDecoder::new(file)?;
    let frames = decoder.into_frames();

    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);
    let mut frame_count: usize = 0;

    for frame in frames {
        let frame = frame?;
        let img = DynamicImage::ImageRgba8(frame.into_buffer());
        let img = img.resize_exact(
            LED_WIDTH as u32,
            LED_HEIGHT as u32,
            image::imageops::FilterType::Nearest,
        );

        for y in 0..LED_HEIGHT {
            for x in 0..LED_WIDTH {
                let pixel = img.get_pixel(x as u32, y as u32);
                let led_data = [0x1F, pixel[2], pixel[1], pixel[0]];
                writer.write_all(&led_data)?;
            }
        }
        frame_count += 1;
    }

    writer.flush()?;
    println!("Converted {} frames", frame_count);
    Ok(frame_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!("SK9822 Animation Converter");
        eprintln!("Usage: {} <input> <output.bin> <framerate>", args[0]);
        eprintln!();
        eprintln!("Input formats:");
        eprintln!("  *.gif         - Animated GIF");
        std::process::exit(1);
    }

    let input = Path::new(GIF_DIR).join(&args[1]);
    let output = Path::new(BIN_DIR)
        .join(&args[2])
        .with_extension(BIN_EXTENSION);

    if !&args[1].ends_with(".gif") {
        eprintln!("Error: Unknown input type. Must be .gif");
        std::process::exit(1);
    }

    let frame_count = gif_to_animation(input, output)?;
    let framerate: u64 = args[3].parse()?;

    let new_animation = Animation::new(
        &args[1].strip_suffix(".gif").unwrap_or(&args[1]),
        framerate,
        frame_count,
    );
    new_animation.save()?;

    println!("Done!");
    Ok(())
}
