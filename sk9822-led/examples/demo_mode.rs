use anyhow::{Ok, Result};
use matrix_animation_player::MatrixAnimationPlayer;
use sk9822_led::*;
use std::time::Duration;

const LED_WIDTH: u8 = 14;
const LED_HEIGHT: u8 = 14;

#[tokio::main]
async fn main() -> Result<()> {
    let mut leds = Sk9822LedMatrix::new(LED_WIDTH, LED_HEIGHT);
    let _ = leds.connect("/dev/spidev0.0");

    let mut player = MatrixAnimationPlayer::new(leds, "color_test");

    loop {
        player.play().await?;
        tokio::time::sleep(Duration::from_secs(12)).await;

        player.change_animation("dog").await?;
        player.play().await?;
        tokio::time::sleep(Duration::from_secs(3)).await;

        player.change_animation("idle").await?;
        player.play().await?;
        tokio::time::sleep(Duration::from_secs(3)).await;

        player.change_animation("rainbow_breathing_shift").await?;
        player.play().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;

        player.change_animation("color_test").await?;
    }

    Ok(())
}
