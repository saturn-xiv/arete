use std::sync::Arc;

use actix::prelude::*;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, Result, State};
use futures::Future;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use mime::{APPLICATION_JSON, TEXT_HTML_UTF_8};

use super::super::{
    app::http::State as AppState,
    request::{ClientIp, Home, Locale, Token},
};
use super::{context::Context, session::Session, Schema};

pub struct GraphQLData {
    pub request: GraphQLRequest,
    pub session: Session,
    pub context: Arc<Context>,
}

impl Message for GraphQLData {
    type Result = Result<String>;
}

pub struct GraphQLExecutor {
    pub schema: Arc<Schema>,
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg
            .request
            .execute(&self.schema, &(msg.context, msg.session));
        let txt = serde_json::to_string(&res)?;
        Ok(txt)
    }
}

pub fn doc(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    let html = graphiql_source(req.path());
    Ok(HttpResponse::Ok()
        .content_type(TEXT_HTML_UTF_8.to_string())
        .body(html))
}

pub fn post(
    (app, body, home, client_ip, locale, token): (
        State<AppState>,
        Json<GraphQLRequest>,
        Home,
        ClientIp,
        Option<Locale>,
        Option<Token>,
    ),
) -> FutureResponse<HttpResponse> {
    app.graphql
        .send(GraphQLData {
            request: body.0,
            context: app.context.clone(),
            session: Session::new(&app.context.clone(), home, client_ip, locale, token),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(it) => Ok(HttpResponse::Ok()
                .content_type(APPLICATION_JSON.to_string())
                .body(it)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
