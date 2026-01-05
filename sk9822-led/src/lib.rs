use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io::Write;

const BRIGHTNESS_CONST: u8 = 0xE0;
const MAX_BRIGHTNESS: u8 = 0xFF;
const MIN_BRIGHTNESS: u8 = BRIGHTNESS_CONST | 0x00;
const BRIGHTNESS_INDEX: usize = 0;
const BLUE_INDEX: usize = 1;
const GREEN_INDEX: usize = 2;
const RED_INDEX: usize = 3;
pub const START_FRAME: [u8; 4] = [0u8; 4];
pub const END_FRAME: [u8; 4] = [0x00u8; 4];

#[derive(Clone, Copy)]
pub struct Sk9822Led {
    data: [u8; 4],
}

pub struct Sk9822LedMatrix {
    rows: u8,
    cols: u8,
    data: Vec<Vec<Sk9822Led>>,
    spi: Option<Spidev>,
}

impl Sk9822Led {
    pub fn new() -> Self {
        Self {
            data: [MIN_BRIGHTNESS, 0, 0, 0],
        }
    }

    pub fn brightness(mut self, brightness: u8) -> Self {
        if brightness > 0x1F {
            self.data[BRIGHTNESS_INDEX] = MAX_BRIGHTNESS;
        } else {
            self.data[BRIGHTNESS_INDEX] = BRIGHTNESS_CONST | brightness;
        }

        self
    }

    pub fn blue(mut self, blue: u8) -> Self {
        self.data[BLUE_INDEX] = blue;

        self
    }

    pub fn green(mut self, green: u8) -> Self {
        self.data[GREEN_INDEX] = green;

        self
    }

    pub fn red(mut self, red: u8) -> Self {
        self.data[RED_INDEX] = red;

        self
    }

    pub fn rgb(mut self, blue: u8, green: u8, red: u8) -> Self {
        self.data[BLUE_INDEX] = blue;
        self.data[GREEN_INDEX] = green;
        self.data[RED_INDEX] = red;

        self
    }

    pub fn get_data(&self) -> &[u8; 4] {
        &self.data
    }

    pub fn update(&mut self, f: impl FnOnce(Self) -> Self) {
        *self = f(*self);
    }
}

impl Sk9822LedMatrix {
    pub fn new(rows: u8, cols: u8) -> Self {
        let data = vec![vec![Sk9822Led::new(); cols as usize]; rows as usize];

        Self {
            rows,
            cols,
            data,
            spi: None,
        }
    }

    pub fn connect(&mut self, path: &str) -> Result<(), std::io::Error> {
        let mut spi = Spidev::open(path)?;

        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(8_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();

        spi.configure(&options)?;

        self.spi = Some(spi);

        Ok(())
    }

    pub fn send_data(&mut self) -> Result<usize, std::io::Error> {
        let mut data = Vec::new();

        data.extend_from_slice(&START_FRAME);

        for row in &self.data {
            for led in row {
                data.extend_from_slice(led.get_data());
            }
        }

        let total_leds = (self.rows as usize) * (self.cols as usize);
        let end_frame_byetes = (total_leds / 16) + 1;
        for _ in 0..end_frame_byetes {
            data.extend_from_slice(&END_FRAME);
        }

        self.spi
            .as_mut()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "SPI not initialized. Use connect()",
                )
            })?
            .write(&data)
    }

    pub fn get(&self, row: u8, col: u8) -> Option<&Sk9822Led> {
        self.data.get(row as usize)?.get(col as usize)
    }

    pub fn get_mut(&mut self, row: u8, col: u8) -> Option<&mut Sk9822Led> {
        self.data.get_mut(row as usize)?.get_mut(col as usize)
    }

    pub fn update(&mut self, row: u8, col: u8, f: impl FnOnce(Sk9822Led) -> Sk9822Led) {
        if let Some(led) = self.get_mut(row, col) {
            led.update(f);
        }
    }
}
