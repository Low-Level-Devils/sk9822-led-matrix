//run using
// cargo run --bin gif_convert --features converter -- sk9822-led/bin/color_test.gif sk9822-led/animations/color_test.bin

use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, GenericImageView};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

const LED_WIDTH: usize = 14;
const LED_HEIGHT: usize = 14;
const BYTES_PER_LED: usize = 4;

fn gif_to_animation(gif_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting GIF: {} -> {}", gif_path, output_path);
    let file = File::open(gif_path)?;
    let decoder = GifDecoder::new(file)?;
    let frames = decoder.into_frames();

    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);
    let mut frame_count = 0;

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
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("SK9822 Animation Converter");
        eprintln!("Usage: {} <input> <output.bin>", args[0]);
        eprintln!();
        eprintln!("Input formats:");
        eprintln!("  *.gif         - Animated GIF");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    if input.ends_with(".gif") {
        gif_to_animation(input, output)?;
    } else {
        eprintln!("Error: Unknown input type. Must be .gif");
        std::process::exit(1);
    }

    println!("Done!");
    Ok(())
}
