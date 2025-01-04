#![warn(missing_docs)]

//! PUImURI trainer for Ohms law, power equation and their combinations
//!
//! This library provides a trainer implementation that creates and solves excercesis

use eyre::{Context, Result};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    ResolveError(ExerciseSolution),
}

/// What type of an excerise is in question?
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum ExerciseType {
    /// By default we handle Ohms law
    #[default]
    OhmsLaw,
    /// Power equation
    Power,
    /// Combination of all three
    Combined,
}

/// What type of variable are we handling?
#[derive(PartialEq, Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Variable {
    /// Voltage or I in volts
    Voltage,
    /// Current or U in amperes
    Current,
    /// By default we handle Resistance or R in Ohms
    #[default]
    Resistance,
    /// Power or P in watts
    Power,
}

/// What type of an unit is the ExerciseSolution unit in
#[derive(Debug, Clone, Serialize)]
pub enum Unit {
    /// Volts
    Volt,
    /// Amperes
    Ampere,
    /// Ohms
    Ohm,
    /// Watts
    Watt,
}

/// Contains the solution and work needed to reach that answer for a spesific Exercise
#[derive(Debug, Clone, Serialize)]
pub struct ExerciseSolution {
    /// Shows the work needed to reach the answer
    pub steps: Vec<String>,
    /// Contains the answer to the exercise
    pub answer: f64,
    /// Unit type of the answer
    pub unit: Unit,
}

/// An Excersise that user must solve or which is to be explained to the user
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Exercise {
    /// Type of the exercise
    pub exercise_type: ExerciseType,
    /// What variable is missing in the equation that user needs to solve
    pub missing_variable: Variable,
    /// What other variables in the equation are already known
    pub given_variables: Vec<(Variable, f64)>,
    /// What is the correct answer for this exercise
    #[serde(skip_serializing)]
    pub correct_answer: Option<f64>,
}

impl Exercise {
    /// Creates a new ExcerciseBuilder that can build a new exercise
    pub fn new() -> ExerciseBuilder {
        ExerciseBuilder::new()
    }

    /// Check the answer
    pub fn check_answer(&self, user_answer: f64, precision: Option<f64>) -> Option<bool> {
        if let Some(correct_answer) = self.correct_answer {
            if let Some(precision) = precision {
                return Some((user_answer - correct_answer).abs() < precision);
            } else {
                return Some((user_answer - correct_answer).abs() < 0.1);
            }
        }
        None
    }

    /// Solves the equation and question, returns with precise solution and work needed to achieve the result
    pub fn solve(&self) -> Result<ExerciseSolution> {
        let mut steps = Vec::new();
        let answer: f64;
        let unit: Unit;

        let get_value = |variable: Variable| -> Result<f64, TrainerError> {
            self.given_variables
                .iter()
                .find(|(v, _)| *v == variable)
                .map(|(_, val)| *val)
                .ok_or(TrainerError::MissingVariable(variable))
        };

        match self.exercise_type {
            ExerciseType::OhmsLaw => match self.missing_variable {
                Variable::Voltage => {
                    let r = get_value(Variable::Resistance)?;
                    let i = get_value(Variable::Current)?;
                    unit = Unit::Volt;
                    steps.push("U = R * I".to_string());
                    steps.push(format!("U = {}Ω * {}A", r, i));
                    answer = r * i;
                }
                Variable::Current => {
                    let u = get_value(Variable::Voltage)?;
                    let r = get_value(Variable::Resistance)?;
                    unit = Unit::Ampere;
                    steps.push("I = U / R".to_string());
                    steps.push(format!("I = {}V / {}Ω", u, r));
                    answer = u / r;
                }
                Variable::Resistance => {
                    let u = get_value(Variable::Voltage)?;
                    let i = get_value(Variable::Current)?;
                    unit = Unit::Ohm;
                    steps.push("R = U / I".to_string());
                    steps.push(format!("R = {}V / {}A", u, i));
                    answer = u / i;
                }
                _ => unreachable!(),
            },
            ExerciseType::Power => match self.missing_variable {
                Variable::Power => {
                    let u = get_value(Variable::Voltage)?;
                    let i = get_value(Variable::Current)?;
                    unit = Unit::Watt;
                    steps.push("P = U * I".to_string());
                    steps.push(format!("P = {}V * {}A", u, i));
                    answer = u * i;
                }
                Variable::Voltage => {
                    let p = get_value(Variable::Power)?;
                    let i = get_value(Variable::Current)?;
                    unit = Unit::Volt;
                    steps.push("U = P / I".to_string());
                    steps.push(format!("U = {}W / {}A", p, i));
                    answer = p / i;
                }
                Variable::Current => {
                    let p = get_value(Variable::Power)?;
                    let u = get_value(Variable::Voltage)?;
                    unit = Unit::Ampere;
                    steps.push("I = P / U".to_string());
                    steps.push(format!("I = {}W / {}V", p, u));
                    answer = p / u;
                }
                _ => unreachable!(),
            },
            ExerciseType::Combined => match self.missing_variable {
                Variable::Power => {
                    let u = get_value(Variable::Voltage)?;
                    let r = get_value(Variable::Resistance)?;
                    unit = Unit::Watt;
                    steps.push("P = U^2 / R".to_string());
                    steps.push(format!("P = {}V^2 / {}Ω", u, r));
                    answer = (u * u) / r;
                }
                Variable::Current => {
                    let p = get_value(Variable::Power)?;
                    let r = get_value(Variable::Resistance)?;
                    unit = Unit::Ampere;
                    steps.push("I = √(P / R)".to_string());
                    steps.push(format!("I = √({}W / {}Ω)", p, r));
                    answer = (p / r).sqrt();
                }
                Variable::Voltage => {
                    let p = get_value(Variable::Power)?;
                    let r = get_value(Variable::Resistance)?;
                    unit = Unit::Volt;
                    steps.push("U = √(P * R)".to_string());
                    steps.push(format!("U = √({}W * {}Ω)", p, r));
                    answer = (p * r).sqrt();
                }
                Variable::Resistance => {
                    let p = get_value(Variable::Power)?;
                    let u = get_value(Variable::Voltage)?;
                    unit = Unit::Ohm;
                    steps.push("R = U^2 / P".to_string());
                    steps.push(format!("R = {}V^2 / {}W", u, p));
                    answer = (u * u) / p;
                }
            },
        }

        let solution = ExerciseSolution {
            steps,
            answer,
            unit,
        };

        if let Some(correct_answer) = self.correct_answer {
            if solution.answer != correct_answer {
                let sol_answ = solution.answer;
                let self_answ = correct_answer;
                return Err(TrainerError::ResolveError(solution))
                .with_context(|| format!("Unable to verify solution properly. Solution answer: {} != Exercise answer {}", sol_answ, self_answ));
            }
        }

        Ok(solution)
    }
}

/// Builder pattern for creating an Exercise
#[derive(Debug)]
pub struct ExerciseBuilder {
    exercise: Exercise,
    voltage_range: (f64, f64),
    current_range: (f64, f64),
    resistance_range: (f64, f64),
    power_range: (f64, f64),
    rng: ThreadRng,
}

impl Default for ExerciseBuilder {
    fn default() -> Self {
        ExerciseBuilder {
            exercise: Exercise::default(),
            voltage_range: (1.0, 240.0),
            current_range: (0.1, 10.0),
            resistance_range: (1.0, 1000.0),
            power_range: (1.0, 2400.0),
            rng: rand::thread_rng(),
        }
    }
}

impl ExerciseBuilder {
    /// Creates a new ExcersiceBuilder with default settings
    pub fn new() -> ExerciseBuilder {
        ExerciseBuilder::default()
    }

    /// Allows alteration of voltage range min and max values
    pub fn set_voltage_range(mut self, min: f64, max: f64) -> Result<Self> {
        if min > max {
            return Err(TrainerError::MinLargerThanMax)
                .with_context(|| format!("min: {}, max: {}", min, max));
        }
        self.voltage_range = (min, max);
        Ok(self)
    }

    /// Allows alteration of current range min and max values
    pub fn set_current_range(mut self, min: f64, max: f64) -> Result<Self> {
        if min > max {
            return Err(TrainerError::MinLargerThanMax)
                .with_context(|| format!("min: {}, max: {}", min, max));
        }
        self.current_range = (min, max);
        Ok(self)
    }

    /// Allows alteration of resistance range min and max values
    pub fn set_resistance_range(mut self, min: f64, max: f64) -> Result<Self> {
        if min > max {
            return Err(TrainerError::MinLargerThanMax)
                .with_context(|| format!("min: {}, max: {}", min, max));
        }
        self.resistance_range = (min, max);
        Ok(self)
    }

    /// Allows alteration of power range min and max values
    pub fn set_power_range(mut self, min: f64, max: f64) -> Result<Self> {
        if min > max {
            return Err(TrainerError::MinLargerThanMax)
                .with_context(|| format!("min: {}, max: {}", min, max));
        }
        self.power_range = (min, max);
        Ok(self)
    }

    /// Alter type of the exercise
    pub fn set_type(mut self, new_type: ExerciseType) -> Self {
        self.exercise.exercise_type = new_type;
        self
    }

    /// Builds and returns an exercise based on the settings in the builder and proto Exercise within it
    pub fn build(mut self) -> Exercise {
        let voltage = self
            .rng
            .gen_range::<f64, _>(self.voltage_range.0..self.voltage_range.1)
            .round()
            / 100.0; // Round to 2 decimal places
        let current = self
            .rng
            .gen_range::<f64, _>(self.current_range.0..self.current_range.1)
            .round()
            / 100.0; // Round to 2 decimal places
        let resistance = self
            .rng
            .gen_range::<f64, _>(self.resistance_range.0..self.resistance_range.1)
            .round()
            / 100.0; // Round to 2 decimal places
        let power = self
            .rng
            .gen_range::<f64, _>(self.power_range.0..self.power_range.1)
            .round()
            / 100.0; // Round to 2 decimal places

        match self.exercise.exercise_type {
            ExerciseType::OhmsLaw => {
                let missing_variable = [Variable::Voltage, Variable::Current, Variable::Resistance]
                    .choose(&mut self.rng)
                    .unwrap();
                let mut given_variables = Vec::new();

                let correct_answer = match missing_variable {
                    Variable::Voltage => {
                        given_variables.push((Variable::Resistance, resistance));
                        given_variables.push((Variable::Current, current));
                        resistance * current
                    }
                    Variable::Current => {
                        given_variables.push((Variable::Voltage, voltage));
                        given_variables.push((Variable::Resistance, resistance));
                        voltage / resistance
                    }
                    Variable::Resistance => {
                        given_variables.push((Variable::Voltage, voltage));
                        given_variables.push((Variable::Current, current));
                        voltage / current
                    }
                    _ => unreachable!(),
                };
                self.exercise.missing_variable = *missing_variable;
                self.exercise.given_variables = given_variables;
                self.exercise.correct_answer = Some(correct_answer);
            }
            ExerciseType::Power => {
                let missing_variable = [Variable::Power, Variable::Voltage, Variable::Current]
                    .choose(&mut self.rng)
                    .unwrap();
                let mut given_variables = Vec::new();
                let correct_answer = match missing_variable {
                    Variable::Power => {
                        given_variables.push((Variable::Voltage, voltage));
                        given_variables.push((Variable::Current, current));
                        voltage * current
                    }
                    Variable::Voltage => {
                        given_variables.push((Variable::Power, power));
                        given_variables.push((Variable::Current, current));
                        power / current
                    }
                    Variable::Current => {
                        given_variables.push((Variable::Power, power));
                        given_variables.push((Variable::Voltage, voltage));
                        power / voltage
                    }
                    _ => unreachable!(),
                };
                self.exercise.missing_variable = *missing_variable;
                self.exercise.given_variables = given_variables;
                self.exercise.correct_answer = Some(correct_answer);
            }
            ExerciseType::Combined => {
                let selection = [
                    (Variable::Voltage, Variable::Resistance, Variable::Power),
                    (Variable::Power, Variable::Resistance, Variable::Current),
                    (Variable::Power, Variable::Voltage, Variable::Resistance),
                ]
                .choose(&mut self.rng)
                .unwrap();
                let mut given_variables = Vec::new();
                let correct_answer = match selection {
                    (Variable::Voltage, Variable::Resistance, Variable::Power) => {
                        given_variables.push((Variable::Voltage, voltage));
                        given_variables.push((Variable::Resistance, resistance));
                        (voltage * voltage) / resistance
                    }
                    (Variable::Power, Variable::Resistance, Variable::Current) => {
                        given_variables.push((Variable::Power, power));
                        given_variables.push((Variable::Resistance, resistance));
                        (power / resistance).sqrt()
                    }
                    (Variable::Power, Variable::Voltage, Variable::Resistance) => {
                        given_variables.push((Variable::Power, power));
                        given_variables.push((Variable::Voltage, voltage));
                        (voltage * voltage) / power
                    }
                    _ => unreachable!(),
                };
                self.exercise.missing_variable = selection.2;
                self.exercise.given_variables = given_variables;
                self.exercise.correct_answer = Some(correct_answer);
            }
        }
        self.exercise.clone() // clone the protype exercise so that it can be re-used when calling build() again
    }

    /// Builds a new exercise with randomized ExerciseType
    pub fn build_with_random_exercisetype(mut self) -> Exercise {
        let exercise_types = [
            ExerciseType::OhmsLaw,
            ExerciseType::Power,
            ExerciseType::Combined,
        ];
        let exercise_type = exercise_types.choose(&mut self.rng).unwrap();
        self.exercise.exercise_type = *exercise_type;
        self.build()
    }
}
