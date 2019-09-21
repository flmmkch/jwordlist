
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ActixPayloadError(actix_web::error::PayloadError),
    ActixClientSendRequestError(actix_web::client::SendRequestError),
    Other(Box<dyn std::error::Error + 'static>),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<actix_web::error::PayloadError> for Error {
    fn from(error: actix_web::error::PayloadError) -> Self {
        Error::ActixPayloadError(error)
    }
}

impl From<actix_web::client::SendRequestError> for Error {
    fn from(error: actix_web::client::SendRequestError) -> Self {
        Error::ActixClientSendRequestError(error)
    }
}

#[allow(dead_code)]
impl Error {
    pub fn from_other<E: std::error::Error + 'static>(error: E) -> Self {
        Self::Other(Box::new(error))
    }
    pub fn error(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::IoError(e) => Some(e),
            Error::ActixPayloadError(_) => None,
            Error::ActixClientSendRequestError(_) => None,
            Error::Other(e) => Some(e.as_ref()),
        }
    }
    pub fn display(&self) -> &dyn std::fmt::Display {
        match self {
            Error::IoError(e) => e,
            Error::ActixPayloadError(e) => e,
            Error::ActixClientSendRequestError(e) => e,
            Error::Other(ref e) => e,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self.display(), f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error().and_then(|e| e.source())
    }
}