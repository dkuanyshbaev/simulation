use std::{error, fmt};

#[derive(Debug)]
pub enum SimulationError {
    Unknown,
    LED,
    DB,
}

pub type SimulationResult<T> = Result<T, SimulationError>;

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SimulationError::Unknown => write!(f, "Unknown error"),
            SimulationError::LED => write!(f, "LED error"),
            SimulationError::DB => write!(f, "DB error"),
        }
    }
}

impl error::Error for SimulationError {
    fn description(&self) -> &str {
        match *self {
            SimulationError::Unknown => "Unknown error",
            SimulationError::LED => "LED error",
            SimulationError::DB => "DB error",
        }
    }
}
