use super::context::Context;

pub struct Root;

#[juniper::object(
    Context = Context,
)]
impl Root {}
