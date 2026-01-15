use anyhow::{Ok, Result};
use axum::{
    Router,
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
};
use matrix_animation_player::MatrixAnimationPlayer;
use serde::Deserialize;
use sk9822_led::*;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

const LED_WIDTH: u8 = 14;
const LED_HEIGHT: u8 = 14;

#[derive(Deserialize)]
struct AnimationReq {
    animation: String,
}

struct AnimationState {
    player: Mutex<MatrixAnimationPlayer>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut leds = Sk9822LedMatrix::new(LED_WIDTH, LED_HEIGHT);
    let _ = leds.connect("/dev/spidev0.0");

    let mut player = MatrixAnimationPlayer::new(leds, "rainbow_breathing_shift");

    player.play().await?;

    let animation_state = Arc::new(AnimationState {
        player: Mutex::new(player),
    });

    let serve_dir = ServeDir::new("static");

    let app = Router::new()
        .nest_service("/", serve_dir)
        .route("/request", post(request_animation))
        .with_state(animation_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("Server started!\nListening.....");

    let _ = axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn request_animation(
    State(animation_state): State<Arc<AnimationState>>,
    Json(body): Json<AnimationReq>,
) -> impl IntoResponse {
    let mut animation_player = animation_state.player.lock().await;

    let _ = animation_player
        .change_animation(body.animation.as_str())
        .await;

    "Ok"
}
