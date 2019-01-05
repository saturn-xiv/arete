use actix_web::HttpRequest;

use super::super::super::State;

pub fn sign_in(req: &HttpRequest<State>) -> &'static str {
    "hello"
}
