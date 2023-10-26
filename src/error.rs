use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed,

    // -- Error Model
    DeleteTicketFailed { id: usize },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        print!("->> {:12} - {self:?}", "INTO_RES");

        (StatusCode::UNAUTHORIZED, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
