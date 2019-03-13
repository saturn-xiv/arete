use actix_web::{HttpRequest, HttpResponse};

use super::super::super::app::http::State as AppState;

pub fn index(req: &HttpRequest<AppState>) -> HttpResponse {
    unimplemented!()
}
