pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Success",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not Found",
        }
    }
}
