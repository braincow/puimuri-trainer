#![warn(missing_docs)]

//! PUImURI trainer for Ohms law, power equation and their combinations
//!
//! This web server serves REST interface for training the "PUImURI" related equations and the frontend code

use puimuri_trainer::equations::{EquationExercise, EquationExerciseSolution};
use rocket::{fs::FileServer, serde::json::Json};
use std::{env, path::PathBuf};

#[macro_use]
extern crate rocket;

#[get("/equation")]
fn equation() -> Json<EquationExercise> {
    let exercise = EquationExercise::new().build_with_random_exercisetype();
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
    let mut rocket = rocket::build();

    let frontend_dir = env::var("PUIMURI_FRONTEND_DIR").unwrap_or("static".to_string());
    let frontend_path = PathBuf::from(frontend_dir);
    if frontend_path.exists() && frontend_path.is_dir() {
        rocket = rocket.mount("/", FileServer::from(frontend_path));
    }

    rocket.mount("/api", routes![equation, equation_answer])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use puimuri_trainer::equations::{EquationExercise, EquationExerciseBuilder};
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json;

    #[test]
    fn equation_endpoint() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/api", super::equation)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        let exercise = response.into_json::<EquationExercise>();
        assert_eq!(exercise.is_some(), true);
    }

    #[test]
    fn equation_answer_endpoint_correct() {
        let exercise = EquationExerciseBuilder::new().build_with_random_exercisetype();
        let answer = exercise.solve().unwrap().answer;

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/api", super::equation_answer(answer)))
            .body(json::to_string(&exercise).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn equation_answer_endpoint_incorrect() {
        let exercise = EquationExerciseBuilder::new().build_with_random_exercisetype();

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!("/api", super::equation_answer(-1.1))) // we should never see in default configuration answers under 0
            .body(json::to_string(&exercise).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::PreconditionFailed);
    }
}
