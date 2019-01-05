use std::ops::Deref;

use actix::prelude::*;
use actix_web::{dev::ConnectionInfo, AsyncResponder, FutureResponse, HttpRequest, HttpResponse};
use futures::Future;

use super::super::super::super::super::{errors::Result, orm::DbExecutor};
use super::super::super::{models::leave_words::Dao as LeaveWordDao, MediaType, State};

#[derive(Deserialize)]
struct New {
    body: String,
    media_type: String,
}

struct Create {
    ip: ConnectionInfo,
    body: String,
    media_type: MediaType,
}

impl Message for Create {
    type Result = Result<()>;
}

impl Handler<Create> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let db = self.0.get()?;
        let db = db.deref();
        LeaveWordDao::add(db, msg.ip.remote(), &msg.body, &msg.media_type)?;
        Ok(())
    }
}

pub fn create(req: &HttpRequest<State>) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(Create {
            body: " ".to_string(),
            media_type: MediaType::TEXT,
            ip: req.connection_info().clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().json(())),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
