use actix_web::{HttpRequest, HttpResponse};

use super::super::super::app::http::State as AppState;

pub fn index(_req: &HttpRequest<AppState>) -> HttpResponse {
    unimplemented!()
}
