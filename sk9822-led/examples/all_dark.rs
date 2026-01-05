use sk9822_led::*;
use std::thread;
use std::time::Duration;

const SLEEP_TIME: Duration = Duration::from_secs(1);

fn main() -> std::io::Result<()> {
    let mut leds = Sk9822LedMatrix::new(14, 14);

    let _ = leds.connect("/dev/spidev0.0");

    loop {
        for row in 0..14 {
            for col in 0..14 {
                leds.update(row, col, |l| l.brightness(0x1F).rgb(0x00, 0x00, 0x00));
            }
        }

        let _ = leds.send_data();

        thread::sleep(SLEEP_TIME);
    }

    Ok(())
}
