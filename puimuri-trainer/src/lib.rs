#![warn(missing_docs)]

//! PUImURI trainer for Ohms law, power equation and their combinations
//!
//! This library provides a trainer implementation that creates and solves excercesis

use equations::{ExerciseSolution, Variable};
use thiserror::Error;

pub mod equations;

/// Error types that this trainer library can return
#[derive(Error, Debug)]
pub enum TrainerError {
    /// Programmer or end-user if allowed by programmer is trying to initialize excersise builder with values that do not make sense
    #[error("Minimum value is larger than maximum value")]
    MinLargerThanMax,
    /// Variable is missing somewhere
    #[error("Variable is missing in definitions")]
    MissingVariable(Variable),
    /// The exercise resolver is getting a different result than what is indicated by the ecercise itself
    #[error("Exercise solution does not match with solved solution")]
    EquationResolveError(ExerciseSolution),
}
