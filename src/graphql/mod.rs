pub mod context;
pub mod mutation;
pub mod query;

use std::num::ParseIntError;
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::Mutex;

use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    GraphQLInputObject, GraphQLObject, ParseScalarResult, ParseScalarValue, RootNode, Value,
};
use mime::{APPLICATION_JSON, TEXT_HTML_UTF_8};

use super::{
    cache::Pool as Cache,
    crypto::Crypto,
    errors::Result,
    jwt::Jwt,
    orm::{Pool as Db, ID as TID},
    plugins::nut::request::CurrentUser,
    queue::rabbitmq::RabbitMQ,
    request::{ClientIp, Locale, Token},
};

pub type Schema = RootNode<'static, query::Query, mutation::Mutation>;

pub const SOURCE: &str = "/graphql";

pub struct ID(pub TID);

impl From<TID> for ID {
    fn from(item: TID) -> Self {
        Self(item)
    }
}

impl FromStr for ID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

juniper::graphql_scalar!(ID where Scalar = <S> {
    description: "ROW ID"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<ID> {
        v.as_scalar_value::<String>()
         .and_then(|s| s.parse().ok())
    }

    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});

pub struct I64(pub i64);

impl From<i64> for I64 {
    fn from(item: i64) -> Self {
        Self(item)
    }
}

impl FromStr for I64 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

juniper::graphql_scalar!(I64 where Scalar = <S> {
    description: "i64"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<I64> {
        v.as_scalar_value::<String>()
         .and_then(|s| s.parse().ok())
    }

    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});

pub struct I16(pub i16);

impl From<i16> for I16 {
    fn from(item: i16) -> Self {
        Self(item)
    }
}

impl FromStr for I16 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

juniper::graphql_scalar!(I16 where Scalar = <S> {
    description: "i16"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<I16> {
        v.as_scalar_value::<String>()
         .and_then(|s| s.parse().ok())
    }

    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});

#[derive(GraphQLObject)]
pub struct Pagination {
    pub size: i32,
    pub page: i32,
    pub total: i32,
}

impl Pagination {
    pub fn new(total: i64, pager: &Pager) -> Self {
        Self {
            total: total as i32,
            page: pager.page,
            size: pager.size,
        }
    }
}

#[derive(GraphQLInputObject, Debug)]
pub struct Pager {
    pub size: i32,
    pub page: i32,
}

impl Pager {
    pub const MIN_SIZE: i64 = 5;
    pub const MAX_SIZE: i64 = 120;
    pub fn offset(&self, total: i64) -> i64 {
        let v = ((self.page as i64) - 1) * (self.size as i64);
        if v < 0 {
            return 0;
        }
        if v >= total {
            return total - (self.size as i64);
        }
        v
    }

    pub fn limit(&self) -> i64 {
        let v = self.size as i64;
        if v < Self::MIN_SIZE {
            Self::MAX_SIZE
        } else {
            v
        }
    }
}

#[derive(GraphQLObject)]
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

type Services = (
    web::Data<Db>,
    web::Data<Cache>,
    web::Data<RabbitMQ>,
    web::Data<Crypto>,
    web::Data<Jwt>,
);
type Params = (Locale, ClientIp, Option<Token>, Option<CurrentUser>);

pub async fn post(
    st: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    services: Services,
    params: Params,
) -> Result<HttpResponse> {
    let db = (services.0).get()?;
    let ch = (services.1).get()?;
    let body = web::block(move || {
        let res = data.execute(
            &st,
            &context::Context {
                db,
                cache: Mutex::new(ch),
                queue: (services.2).into_inner(),
                crypto: (services.3).into_inner(),
                jwt: (services.4).into_inner(),
                locale: (params.0).0,
                client_ip: (params.1).0,
                token: (params.2).map(|it| it.0),
                current_user: (params.3).map(|it| it.0),
            },
        );
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .body(body))
}
