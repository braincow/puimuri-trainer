use puimuri_trainer::{ExcerciseBuilder, Exercise, ExerciseSolution};
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[get("/exercise")]
fn exercise() -> Json<Exercise> {
    let excercise_builder = ExcerciseBuilder::default();
    let excercise = excercise_builder.build_with_random_excercisetype();
    Json(excercise)
}

#[derive(Responder)]
enum AnswerResponder {
    #[response(status = 412, content_type = "json")] // precondition failed
    IncorrectAnswer(Json<ExerciseSolution>),
    #[response(status = 200, content_type = "json")] // ok
    CorrectAnswer(Json<ExerciseSolution>),
}

#[post("/exercise/answer/<answer>", data = "<exercise>")]
fn answer(answer: f64, exercise: Json<Exercise>) -> AnswerResponder {
    let solution = exercise.solve().unwrap();
    if answer == solution.answer {
        AnswerResponder::CorrectAnswer(Json(solution))
    } else {
        AnswerResponder::IncorrectAnswer(Json(solution))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![exercise, answer])
}
