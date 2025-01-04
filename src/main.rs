#![warn(missing_docs)]

//! PUImURI trainer for Ohms law, power equation and their combinations
//!
//! This web server serves REST interface for training the "PUImURI" related equations and the frontend code

use puimuri_trainer::equations::{Exercise, ExerciseBuilder, ExerciseSolution};
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[get("/equation")]
fn exercise() -> Json<Exercise> {
    let excercise_builder = ExerciseBuilder::default();
    let excercise = excercise_builder.build_with_random_exercisetype();
    Json(excercise)
}

#[derive(Responder)]
enum AnswerResponder {
    #[response(status = 412, content_type = "json")] // precondition failed
    IncorrectAnswer(Json<ExerciseSolution>),
    #[response(status = 200, content_type = "json")] // ok
    CorrectAnswer(Json<ExerciseSolution>),
}

#[post("/equation/answer/<answer>", data = "<exercise>")]
fn answer(answer: f64, exercise: Json<Exercise>) -> AnswerResponder {
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
    rocket::build().mount("/api", routes![exercise, answer])
}
