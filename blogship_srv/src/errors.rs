use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
    ServerError(String),
}

impl From<rocket::Error> for AppError {
    fn from(re: rocket::Error) -> Self {
        Self::ServerError(re.to_string())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ServerError(e) => write!(f, "{}", e),
        }
    }
}
