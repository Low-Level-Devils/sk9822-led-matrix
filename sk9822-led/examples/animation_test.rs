use sk9822_led::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

const FPS: u64 = 1;
const SLEEP_TIME: Duration = Duration::from_millis(1000 / FPS);
const LED_WIDTH: u8 = 14;
const LED_HEIGHT: u8 = 14;
const FRAMES: usize = 12;

fn main() -> std::io::Result<()> {
    let mut file = File::open("animations/color_test.bin").expect("color_test.bin not found");
    let mut buffer = [0u8; 4];

    let mut leds = Sk9822LedMatrix::new(LED_WIDTH, LED_HEIGHT);
    let _ = leds.connect("/dev/spidev0.0");

    loop {
        for _frame in 0..FRAMES {
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

            thread::sleep(SLEEP_TIME);
        }

        file.seek(SeekFrom::Start(0))
            .expect("Failed to rewind animation file");
    }

    Ok(())
}
