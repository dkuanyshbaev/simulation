use diesel::result::ConnectionError;
use diesel::result::Error as DieselError;
use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum SimulationError {
    LED,
    DB,
}

pub type SimulationResult<T> = Result<T, SimulationError>;

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SimulationError::LED => write!(f, "LED error"),
            SimulationError::DB => write!(f, "DB error"),
        }
    }
}

impl error::Error for SimulationError {
    fn description(&self) -> &str {
        match *self {
            SimulationError::LED => "LED error",
            SimulationError::DB => "DB error",
        }
    }
}

impl From<ConnectionError> for SimulationError {
    fn from(error: ConnectionError) -> Self {
        match error {
            _ => SimulationError::DB,
        }
    }
}

impl From<DieselError> for SimulationError {
    fn from(error: DieselError) -> Self {
        match error {
            _ => SimulationError::DB,
        }
    }
}
