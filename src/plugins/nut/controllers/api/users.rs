use actix_web::HttpRequest;

use super::super::super::State;

pub fn get_sign_in(req: &HttpRequest<State>) -> &'static str {
    "hello"
}
