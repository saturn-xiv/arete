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

    #[fail(display = "bad media type {}", _0)]
    BadMediaType(String),
    #[fail(display = "bad gender {}", _0)]
    BadGender(String),
    #[fail(display = "sodium init failed")]
    SodiumInit,
    #[fail(display = "bad key length, must be 32")]
    SodiumBadKey,
    #[fail(display = "build hash failed")]
    SodiumHash,
    #[fail(display = "decrypt failed")]
    SodiumDecrypt,
    #[fail(display = "bad nonce")]
    SodiumBadNonce,
    #[fail(display = "bad password")]
    UserBadPassword,
    #[fail(display = "your account isn't confirmed")]
    UserIsNotConfirmed,
    #[fail(display = "your account id deleted")]
    UserIsDeleted,
    #[fail(display = "your account is locked")]
    UserIsLocked,
    #[fail(display = "empty message id")]
    RabbitMQEmptyMessageId,
    #[fail(display = "empty message content type")]
    RabbitMQEmptyContentType,
    #[fail(display = "bad message content type {}", _0)]
    RabbitMQBadContentType(String),
    #[fail(display = "{}", _0)]
    Http(hyper::StatusCode),
}
