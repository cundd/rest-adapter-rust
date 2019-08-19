use std::fmt;
use std::error::Error as StdError;

macro_rules! throw {
    ($error:expr) => {return Err($error);};
}

#[derive(Debug)]
pub enum CliError {
    InputError(String),
    OutputError(String),
    RestLibError(Box<dyn StdError>),
    #[allow(unused)]
    UnknownError(String),
}

impl CliError {
    pub fn input_error<S: Into<String>>(description: S) -> Self {
        CliError::InputError(description.into())
    }

    pub fn output_error<S: Into<String>>(description: S) -> Self {
        CliError::OutputError(description.into())
    }

    #[allow(unused)]
    pub fn unknown_error<S: Into<String>>(description: S) -> Self {
        CliError::UnknownError(description.into())
    }
}

impl StdError for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            CliError::InputError(e) => write!(f, "{}", e),
            CliError::OutputError(e) => write!(f, "{}", e),
            CliError::RestLibError(ref e) => write!(f, "{}", e),
            CliError::UnknownError(e) => write!(f, "{}", e),
        }
    }
}

impl From<rest_adapter::Error> for CliError {
    fn from(error: rest_adapter::Error) -> Self {
        CliError::RestLibError(Box::new(error))
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::output_error(StdError::description(&error))
    }
}
