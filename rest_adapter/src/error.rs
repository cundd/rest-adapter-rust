use std::error::Error as StdError;
use reqwest::Error as ReqwestError;
use std::fmt::{Formatter, Display, Error as FmtError};

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError>
}

impl Error {
    pub fn url_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::UrlError(description.into()))
    }

    pub fn http_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::HttpError(description.into()))
    }

    pub fn timeout_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::TimeoutError(description.into()))
    }

    pub fn server_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::ServerError(description.into()))
    }

    pub fn bad_request_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::BadRequestError(description.into()))
    }

    pub fn not_found_error<S: Into<String>>(description: S) -> Self {
        Error::new(Kind::NotFoundError(description.into()))
    }

    pub fn data_error_with_url<E: StdError, S: Into<String>>(error: &E, url: S) -> Self {
        let message = format!("Response data could not be parsed for URL '{}'", url.into());

        Error::new(Kind::DataError(format!("{}: {}", message, error)))
    }

    pub fn reqwest_data_error_with_url<S: Into<String>>(error: ReqwestError, url: S) -> Self {
        let message = format!("Response data could not be parsed for URL '{}'", url.into());

        Self::reqwest_data_error_with_message(error, message)
    }

    pub fn reqwest_data_error_with_message<S: Into<String>>(error: ReqwestError, message: S) -> Self {
        let message_string = message.into();

        match error.get_ref() {
            None => Error::new(Kind::DataError(message_string)),
            Some(err) => Error::new(Kind::DataError(format!("{}: {}", message_string, err)))
        }
    }

    pub fn from_error<E: StdError + 'static>(error: E) -> Self {
        Error { inner: Box::new(error) }
    }

    fn new(kind: Kind) -> Self {
        Error { inner: Box::new(kind) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.inner)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.as_ref())
    }
}

#[inline]
fn error_url(e: &ReqwestError) -> String {
    match e.url() {
        None => "".to_owned(),
        Some(url) => url.clone().into_string()
    }
}

#[inline]
fn error_status(e: &ReqwestError) -> u16 {
    match e.status() {
        None => 0,
        Some(s) => s.as_u16()
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        let suffix = format!(" '{}' (Status code: {})", error_url(&error), error_status(&error));

        return match error {
            _ if error.is_http() => Error::new(Kind::HttpError(format!("Error making request to {}", suffix))),
            _ if error.is_timeout() => Error::new(Kind::TimeoutError(format!("Timeout during request to {}", suffix))),
            _ if error.is_redirect() => Error::new(Kind::RedirectError(format!("Too many redirects for URL '{}'", error_url(&error)))),
            _ if error.is_client_error() => Error::new(Kind::BadRequestError(format!("Invalid request to {}", suffix))),
            _ if error.is_server_error() => Error::new(Kind::ServerError(format!("Server error for request to {}", suffix))),
            _ if error.is_serialization() => Error::reqwest_data_error_with_message(error, "Response data could not be parsed"),
            _ => Error::from_error(error)
        };
//        if error.is_http() {}
//
//        if error.is_timeout() {
//            Error::new(Kind::TimeoutError(
//                format!(
//                    "Timeout during request to: '{}' (Status code: {})",
//                    error_url(error),
//                    error_status(error)
//                )
//            ))
//        }

//        if error.is_serialization() {
//            let message = "Response data could not be parsed";
//            return match error.get_ref() {
//                None => Error::new(Kind::DataError(message.to_owned())),
//                Some(err) => Error::new(Kind::DataError(format!("{}: {}", message, err)))
//            };
//        }

//        if error.is_redirect() {
//            return Error::new(Kind::RedirectError(format!("Too many redirects for URL '{}'", error_url(error))));
//        }

//        if error.is_client_error() {
//            return Error::new(Kind::BadRequestError(
//                format!(
//                    "Invalid request to: '{}' (Status code: {})",
//                    error_url(error),
//                    error_status(error)
//                )
//            ));
//        }
//
//        if error.is_server_error() {
//            return Error::new(Kind::ServerError(
//                format!(
//                    "Server error for request to: '{}' (Status code: {})",
//                    error_url(error),
//                    error_status(error)
//                )
//            ));
//        }
//        Error::from_error(error)
    }
}

impl From<::url::ParseError> for Error {
    fn from(error: ::url::ParseError) -> Self {
        Error::from_error(error)
    }
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error::from_error(error)
    }
}

#[derive(Debug)]
enum Kind {
    UrlError(String),
    HttpError(String),
    TimeoutError(String),
    DataError(String),
    RedirectError(String),
    BadRequestError(String),
    NotFoundError(String),
    ServerError(String),
}

impl StdError for Kind {}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Kind::UrlError(s) => write!(f, "URL error: {}", s),
            Kind::HttpError(s) => write!(f, "HTTP error: {}", s),
            Kind::TimeoutError(s) => write!(f, "Timeout error: {}", s),
            Kind::DataError(s) => write!(f, "Data error: {}", s),
            Kind::RedirectError(s) => write!(f, "Redirect error: {}", s),
            Kind::BadRequestError(s) => write!(f, "Bad-Request error: {}", s),
            Kind::NotFoundError(s) => write!(f, "Not-Found error: {}", s),
            Kind::ServerError(s) => write!(f, "Server error: {}", s),
        }
    }
}
