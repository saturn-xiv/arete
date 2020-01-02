pub mod context;
pub mod mutation;
pub mod query;

use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    RootNode,
};
use mime::{APPLICATION_JSON, TEXT_HTML_UTF_8};

pub type Schema = RootNode<'static, query::Root, mutation::Root>;

pub const SOURCE: &'static str = "/graphql";

pub async fn get() -> HttpResponse {
    let html = graphiql_source(SOURCE);
    HttpResponse::Ok()
        .content_type(TEXT_HTML_UTF_8.to_string())
        .body(html)
}

pub async fn post(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let body = web::block(move || {
        let res = data.execute(&st, &context::Context {});
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body(body))
}
