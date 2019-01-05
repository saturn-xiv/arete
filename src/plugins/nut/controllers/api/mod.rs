pub mod leave_words;
pub mod users;

use actix_web::{HttpRequest, Json, Result};

use super::super::State;

pub fn install(req: &HttpRequest<State>) -> Result<Json<()>> {
    Ok(Json(()))
}
