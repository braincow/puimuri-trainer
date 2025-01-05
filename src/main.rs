#![warn(missing_docs)]

//! PUImURI trainer for Ohms law, power equation and their combinations
//!
//! This web server serves REST interface for training the "PUImURI" related equations and the frontend code

use puimuri_trainer::equations::{
    EquationExercise, EquationExerciseBuilder, EquationExerciseSolution,
};
use rocket::{fs::FileServer, serde::json::Json};
use std::env;

#[macro_use]
extern crate rocket;

#[get("/equation")]
fn equation() -> Json<EquationExercise> {
    let exercise_builder = EquationExerciseBuilder::default();
    let exercise = exercise_builder.build_with_random_exercisetype();
    Json(exercise)
}

#[derive(Responder)]
enum AnswerResponder {
    #[response(status = 412, content_type = "json")] // precondition failed
    IncorrectAnswer(Json<EquationExerciseSolution>),
    #[response(status = 200, content_type = "json")] // ok
    CorrectAnswer(Json<EquationExerciseSolution>),
}

#[post("/equation/answer/<answer>", data = "<exercise>")]
fn equation_answer(answer: f64, exercise: Json<EquationExercise>) -> AnswerResponder {
    let solution = exercise.solve().unwrap();
    if (answer - solution.answer).abs() < 0.01 {
        AnswerResponder::CorrectAnswer(Json(solution))
    } else {
        AnswerResponder::IncorrectAnswer(Json(solution))
    }
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build()
        .mount("/api", routes![equation, equation_answer])
        .mount(
            "/",
            FileServer::from(env::var("PUIMURI_FRONTEND_DIR").unwrap_or("static".to_string())),
        )
}
