use std::result::Result as StdResult;

use failure::{Error as FailureError, Fail};

pub type Result<T> = StdResult<T, FailureError>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),
    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] std::str::Utf8Error),
    #[fail(display = "{}", _0)]
    NetAddrParse(#[fail(cause)] std::net::AddrParseError),

    #[fail(display = "{}", _0)]
    Nix(#[fail(cause)] nix::Error),
    #[fail(display = "{}", _0)]
    Git(#[fail(cause)] git2::Error),
    #[fail(display = "{}", _0)]
    Diesel(#[fail(cause)] diesel::result::Error),
    #[fail(display = "{}", _0)]
    SerdeJson(#[fail(cause)] serde_json::Error),
    #[fail(display = "{}", _0)]
    MimeFromStr(#[fail(cause)] mime::FromStrError),
    #[fail(display = "{}", _0)]
    LettreSmtp(#[fail(cause)] lettre::smtp::error::Error),
    #[fail(display = "{}", _0)]
    R2d2(#[fail(cause)] r2d2::Error),
    #[fail(display = "{}", _0)]
    Multipart(actix_multipart::MultipartError),

    #[fail(display = "{}", _0)]
    Http(actix_web::http::StatusCode),
    #[fail(display = "{}", _0)]
    Rss(String),
}
