pub mod context;
pub mod mutation;
pub mod query;

use actix_web::{web, Error, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    RootNode,
};
use mime::{APPLICATION_JSON, TEXT_HTML_UTF_8};

pub type Schema = RootNode<'static, query::Query, mutation::Mutation>;

pub const SOURCE: &str = "/graphql";

#[derive(juniper::GraphQLObject)]
#[graphql(description = "OK!")]
pub struct OK {
    pub created_at: NaiveDateTime,
}
impl Default for OK {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_local(),
        }
    }
}

pub async fn get() -> HttpResponse {
    let html = graphiql_source(SOURCE);
    HttpResponse::Ok()
        .content_type(TEXT_HTML_UTF_8.to_string())
        .body(html)
}

pub async fn post(
    st: web::Data<Schema>,
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
