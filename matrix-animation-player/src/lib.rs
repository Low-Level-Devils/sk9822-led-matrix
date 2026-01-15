use anyhow::{Ok, Result};
use matrix_animation::Animation;
use sk9822_led::Sk9822LedMatrix;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::task::JoinHandle;

pub struct MatrixAnimationPlayer {
    matrix: Arc<Mutex<Sk9822LedMatrix>>,
    animation: Animation,
    playing_mutex: Arc<Mutex<bool>>,
    play_handle: Option<JoinHandle<Result<()>>>,
}

impl MatrixAnimationPlayer {
    const METADATA_PATH_RUNTIME: &str = "static/animations.json";

    pub fn new(matrix: Sk9822LedMatrix, animation: &str) -> Self {
        Self {
            matrix: Arc::new(Mutex::new(matrix)),
            animation: Self::load_animation(animation),
            playing_mutex: Arc::new(Mutex::new(false)),
            play_handle: None,
        }
    }

    pub async fn play(&mut self) -> Result<()> {
        self.stop().await?;

        let mut file = self.animation.load()?;
        let sleep_time = self.animation.get_time_per_frame();

        let frames = self.animation.frames();
        let matrix = self.matrix.clone();

        *self.playing_mutex.lock().unwrap() = true;

        let playing_mutex = self.playing_mutex.clone();

        let handle = tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 4];

            loop {
                for _frame in 0..frames {
                    if !*playing_mutex.lock().unwrap() {
                        return Ok(());
                    }

                    {
                        let mut mat = matrix.lock().unwrap();

                        for row in 0..mat.rows() {
                            for col in 0..mat.cols() {
                                let target_col = if (row % 2) == 0 {
                                    col
                                } else {
                                    mat.cols() - 1 - col
                                };

                                match file.read_exact(&mut buffer) {
                                    std::io::Result::Ok(_) => {
                                        mat.update(row, target_col, |led| {
                                            led.brightness(buffer[0])
                                                .rgb(buffer[1], buffer[2], buffer[3])
                                        });
                                    }
                                    Err(e) => {
                                        return Err(anyhow::anyhow!(
                                            "Failed to read frame data: {}",
                                            e
                                        ));
                                    }
                                }
                            }
                        }

                        mat.send_data()?;
                    }

                    thread::sleep(sleep_time);
                }

                file.seek(SeekFrom::Start(0))
                    .expect("Failed to rewind animation file");
            }
        });

        self.play_handle = Some(handle);

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        *self.playing_mutex.lock().unwrap() = false;

        if let Some(handle) = self.play_handle.take() {
            handle.await??;
        }

        Ok(())
    }

    pub fn load_animation(animation: &str) -> Animation {
        let path = Path::new(Self::METADATA_PATH_RUNTIME);
        let json_string = std::fs::read_to_string(path).expect("Failed to read animation metadata");
        let animations: HashMap<String, Animation> =
            serde_json::from_str(&json_string).expect("Failed to parse animation metadata");
        animations
            .get(animation)
            .expect("Animation not found")
            .clone()
    }

    pub async fn change_animation(&mut self, animation: &str) -> Result<()> {
        self.stop().await?;
        self.animation = Self::load_animation(animation);
        self.play().await?;
        Ok(())
    }
}
