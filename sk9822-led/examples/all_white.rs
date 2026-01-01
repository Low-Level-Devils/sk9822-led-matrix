use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io::Write;

const START_FRAME: [u8; 4] = [0u8; 4];
const END_FRAME: [u8; 4] = [0xFFu8; 4];
const UNIVERSAL_BRIGHTNESS: u8 = 0xE0 | 0x1F;

fn main() -> std::io::Result<()> {
    // Open the SPI device (typically /dev/spidev0.0 or /dev/spidev0.1)
    let mut spi = Spidev::open("/dev/spidev0.0")?;

    // Configure SPI options
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(400_000) // 1 MHz - adjust based on your matrix specs
        .mode(SpiModeFlags::SPI_MODE_0) // Most LED matrices use MODE_0
        .build();

    spi.configure(&options)?;

    // Send data to your LED matrix
    let mut data = Vec::new();

    data.extend_from_slice(&START_FRAME);

    for _i in 0..15 {
        data.extend_from_slice(&[UNIVERSAL_BRIGHTNESS, 0xFF, 0xFF, 0xFF]); // LED data
    }

    data.extend_from_slice(&END_FRAME);

    spi.write(&data)?;

    Ok(())
}
