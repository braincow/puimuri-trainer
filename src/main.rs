#![warn(missing_docs)]

//! PUImURI trainer for Ohms law, power equation and their combinations
//!
//! This web server serves REST interface for training the "PUImURI" related equations and the frontend code

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use eyre::Result;
use puimuri_trainer::equations::{EquationExercise, EquationExerciseSolution};
use std::env;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let frontend_dir = env::var("PUIMURI_FRONTEND_DIR").unwrap_or("static".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let address = env::var("ADDRESS").unwrap_or("127.0.0.1".to_string());

    let app = Router::new()
        .fallback_service(ServeDir::new(frontend_dir))
        .route("/api/equation", get(equation))
        .route("/api/equation/answer/{answer}", post(equation_answer))
        .layer(TraceLayer::new_for_http());

    let listener =
        tokio::net::TcpListener::bind(format!("{address}:{port}", address = address, port = port))
            .await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn equation() -> (StatusCode, Json<EquationExercise>) {
    let exercise = EquationExercise::builder().build_with_random_exercisetype();
    (StatusCode::OK, Json(exercise))
}

async fn equation_answer(
    Path(answer): Path<f64>,
    Json(exercise): Json<EquationExercise>,
) -> (StatusCode, Json<EquationExerciseSolution>) {
    let solution = exercise.solve().unwrap();
    if (answer - solution.answer).abs() < 0.01 {
        return (StatusCode::OK, Json(solution)); // answer is correct within certain decimal point
    }
    (StatusCode::PRECONDITION_FAILED, Json(solution)) // answer is way off or incorrect
}
