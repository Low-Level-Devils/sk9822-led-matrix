use sk9822_led::*;
use std::thread;
use std::time::Duration;

const SLEEP_TIME: Duration = Duration::from_millis(35);

fn main() -> std::io::Result<()> {
    let mut leds = Sk9822LedMatrix::new(1, 14);

    let _ = leds.connect("/dev/spidev0.0");

    for col in 0..14 {
        leds.update(0, col, |l| l.brightness(0x00).rgb(0, 0, 0));
    }

    let _ = leds.send_data();

    let mut led_1: u8 = 7;
    let mut led_1_prev: u8 = 6;
    let mut led_2: u8 = 7;
    let mut led_2_prev: u8 = 8;

    let mut led_1_increasing = true;
    let mut led_2_increasing = false;

    loop {
        leds.update(0, led_1, |l| l.brightness(0x1F).red(0xFF));
        leds.update(0, led_2, |l| l.brightness(0x1F).green(0xFF));
        leds.update(0, led_1_prev, |l| l.brightness(0x00).rgb(0, 0, 0));
        leds.update(0, led_2_prev, |l| l.brightness(0x00).rgb(0, 0, 0));

        if led_1 == 13 {
            led_1_increasing = false;
        } else if led_1 == 0 {
            led_1_increasing = true;
        }

        if led_2 == 13 {
            led_2_increasing = false;
        } else if led_2 == 0 {
            led_2_increasing = true;
        }

        led_1_prev = led_1;
        led_2_prev = led_2;

        if led_1_increasing {
            led_1 += 1;
        } else {
            led_1 -= 1;
        }

        if led_2_increasing {
            led_2 += 1;
        } else {
            led_2 -= 1;
        }

        let _ = leds.send_data();

        thread::sleep(SLEEP_TIME);
    }

    Ok(())
}
