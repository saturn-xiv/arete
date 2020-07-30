pub mod context;
pub mod mutation;
pub mod query;

use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    RootNode,
};
use mime::{APPLICATION_JSON, TEXT_HTML_UTF_8};

use super::{
    cache::Pool as Cache,
    crypto::Crypto,
    errors::Result,
    jwt::Jwt,
    orm::Pool as Db,
    plugins::nut::request::CurrentUser,
    queue::rabbitmq::RabbitMQ,
    request::{ClientIp, Locale, Token},
};

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
    services: (
        web::Data<Db>,
        web::Data<Cache>,
        web::Data<RabbitMQ>,
        web::Data<Crypto>,
        web::Data<Jwt>,
    ),
    params: (Locale, ClientIp, Option<Token>, Option<CurrentUser>),
) -> Result<HttpResponse> {
    let db = (services.0).get()?;
    let ch = (services.1).get()?;
    let body = web::block(move || {
        let res = data.execute(
            &st,
            &context::Context {
                db,
                cache: ch,
                queue: (services.2).into_inner(),
                crypto: (services.3).into_inner(),
                jwt: (services.4).into_inner(),
                locale: (params.0).0,
                client_ip: (params.1).0,
                token: (params.2).map(|it| it.0),
                user: (params.3).map(|it| it.0),
            },
        );
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body(body))
}
