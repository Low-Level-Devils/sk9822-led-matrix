use matrix_animation_player::MatrixAnimationPlayer;
use sk9822_led::*;

const LED_WIDTH: u8 = 14;
const LED_HEIGHT: u8 = 14;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("SK9822 Animation Test");
        eprintln!("Usage: {} <input> ", args[0]);
        std::process::exit(1);
    }

    let mut leds = Sk9822LedMatrix::new(LED_WIDTH, LED_HEIGHT);
    let _ = leds.connect("/dev/spidev0.0");

    let mut player = MatrixAnimationPlayer::new(leds, &args[1]);

    player.play()?;

    Ok(())
}
