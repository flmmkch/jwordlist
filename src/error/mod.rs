#[derive(Debug, Clone)]
pub struct JWordListErrorResponse<E: std::error::Error> {
    pub error: E,
}

impl<E: std::error::Error> std::fmt::Display for JWordListErrorResponse<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.error)
    }
}

impl<E: std::error::Error> actix_web::error::ResponseError for JWordListErrorResponse<E> {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().body(format!("{}", &self.error))
    }
    fn render_response(&self) -> actix_web::HttpResponse {
        self.error_response()
    }
}

impl<E: std::error::Error> From<E> for JWordListErrorResponse<E> {
    fn from(error: E) -> Self {
        JWordListErrorResponse { error }
    }
}
