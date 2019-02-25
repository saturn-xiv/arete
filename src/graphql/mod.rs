pub mod context;
pub mod mutation;
pub mod query;
pub mod session;

use std::sync::Arc;

use juniper::{
    parser::{ParseError, ScalarToken, Token},
    ParseScalarResult, Value,
};
use serde::Serialize;

use super::errors::Result;

pub type Schema =
    juniper::RootNode<'static, query::Query, mutation::Mutation, juniper::DefaultScalarValue>;
pub type Context = (Arc<context::Context>, session::Session);

pub trait Handler {
    type Item;
    fn handle(&self, c: &context::Context, s: &session::Session) -> Result<Self::Item>;
}

#[derive(Serialize)]
pub struct I64(pub i64);

impl From<I64> for i64 {
    fn from(v: I64) -> i64 {
        v.0
    }
}

graphql_scalar!(I64 as "I64" where Scalar = <S> {
    description: "For PostgreSql BIGSERIAL type"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<I64> {
        if let Some(v) = v.as_scalar_value::<String>(){
            if let Ok(v) = v.parse::<i64>(){
                return Some(I64(v));
            }
        }
        None
    }

    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        if let ScalarToken::String(value) =  value {
            Ok(S::from(value.to_owned()))
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
});
