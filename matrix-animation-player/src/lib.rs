use matrix_animation::Animation;
use sk9822_led::Sk9822LedMatrix;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::thread;

pub struct MatrixAnimationPlayer {
    matrix: Sk9822LedMatrix,
    animation: Animation,
}

impl MatrixAnimationPlayer {
    const METADATA_PATH_RUNTIME: &str = "animations/metadata/animations.json";

    pub fn new(matrix: Sk9822LedMatrix, animation: &str) -> Self {
        let path = Path::new(Self::METADATA_PATH_RUNTIME);
        let json_string = std::fs::read_to_string(path).expect("Failed to read animation metadata");
        let animations: HashMap<String, Animation> =
            serde_json::from_str(&json_string).expect("Failed to parse animation metadata");
        let animation = animations.get(animation).expect("Animation not found");

        Self {
            matrix,
            animation: animation.clone(),
        }
    }

    pub fn play(&mut self) -> std::io::Result<()> {
        let mut file = self.animation.load()?;

        let mut buffer = [0u8; 4];
        let sleep_time = self.animation.get_time_per_frame();

        loop {
            for _frame in 0..self.animation.frames() {
                for row in 0..self.matrix.rows() {
                    for col in 0..self.matrix.cols() {
                        match file.read_exact(&mut buffer) {
                            Ok(_) => {
                                self.matrix.update(row, col, |led| {
                                    led.brightness(buffer[0])
                                        .rgb(buffer[1], buffer[2], buffer[3])
                                });
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }

                self.matrix.send_data()?;

                thread::sleep(sleep_time);
            }

            file.seek(SeekFrom::Start(0))
                .expect("Failed to rewind animation file");
        }
    }
}
