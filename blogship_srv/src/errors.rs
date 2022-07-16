use std::fmt::Display;

// Notes:
// - `AppError` enum implements the `std::error::Error` trait
//   by implementing `Debug` (through derive) and `Display` (explicit).
// - For all the other errors be "casted" to `AppError`, `From` trait is implemented.

#[derive(Debug)]
pub enum AppError {
    ServerError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ServerError(e) => write!(f, "{}", e),
        }
    }
}

impl From<rocket::Error> for AppError {
    fn from(re: rocket::Error) -> Self {
        Self::ServerError(re.to_string())
    }
}
