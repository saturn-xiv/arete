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
    type Item: Serialize;
    fn handle(&self, c: &context::Context, s: &session::Session) -> Result<Self::Item>;
}

#[derive(Serialize)]
pub struct BigSerial(pub i64);

impl From<BigSerial> for i64 {
    fn from(v: BigSerial) -> i64 {
        v.0
    }
}

graphql_scalar!(BigSerial as "BigSerial" where Scalar = <S> {
    description: "For PostgreSql BIGSERIAL type"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<BigSerial> {
        if let Some(v) = v.as_scalar_value::<String>(){
            if let Ok(v) = v.parse::<i64>(){
                return Some(BigSerial(v));
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
