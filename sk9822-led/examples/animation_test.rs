use sk9822_led::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

const FPS: u64 = 1;
const FRAMES: usize = 1;
const LED_WIDTH: u8 = 14;
const LED_HEIGHT: u8 = 14;
const ANIM_DIR: &str = "animations/";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("SK9822 Animation Test");
        eprintln!("Usage: {} <input> <frames> <optional fps>", args[0]);
        std::process::exit(1);
    }

    if !args[1].ends_with(".bin") {
        eprintln!("Unsupported input format: {}", args[1]);
        std::process::exit(1);
    }

    let frames: usize = args[2].parse().unwrap_or(FRAMES);
    let fps: u64 = if args.len() >= 4 {
        args[3].parse().unwrap_or(FPS)
    } else {
        FPS
    };

    let sleep_time: Duration = Duration::from_millis(1000 / fps);

    let mut file = File::open(ANIM_DIR.to_string() + &args[1]).expect("File not found");
    let mut buffer = [0u8; 4];

    let mut leds = Sk9822LedMatrix::new(LED_WIDTH, LED_HEIGHT);
    let _ = leds.connect("/dev/spidev0.0");

    loop {
        for _frame in 0..frames {
            for row in 0..LED_WIDTH {
                for col in 0..LED_HEIGHT {
                    match file.read_exact(&mut buffer) {
                        Ok(_) => {
                            leds.update(row, col, |led| {
                                led.brightness(buffer[0])
                                    .rgb(buffer[1], buffer[2], buffer[3])
                            });
                        }
                        Err(e) => return Err(e),
                    }
                }
            }

            leds.send_data()?;

            thread::sleep(sleep_time);
        }

        file.seek(SeekFrom::Start(0))
            .expect("Failed to rewind animation file");
    }

    Ok(())
}
