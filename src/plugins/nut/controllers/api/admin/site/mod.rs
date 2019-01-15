pub mod author;
pub mod info;
pub mod seo;
pub mod smtp;
pub mod status;

use std::ops::Deref;
use std::sync::Arc;

use r2d2_redis::redis::cmd;
use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;

use super::super::super::super::super::super::{
    errors::Result,
    orm::Database,
    queue::{rabbitmq::RabbitMQ, Queue},
    redis::Redis,
};
use super::super::super::super::{
    models::user::Dao as UserDao, request::Administrator, tasks::send_email,
};

#[delete("/admin/site/clear-cache")]
pub fn clear_cache(_user: Administrator, redis: Redis) -> Result<Json<String>> {
    let rst = cmd("flushdb").query::<String>(redis.deref())?;
    Ok(Json(rst))
}

#[post("/admin/site/send-test-email")]
pub fn send_test_email(
    user: Administrator,
    db: Database,
    queue: State<Arc<RabbitMQ>>,
) -> Result<Json<()>> {
    let db = db.deref();
    let user = UserDao::by_id(db, &user.id)?;
    queue.publish(
        send_email::NAME.to_string(),
        Uuid::new_v4().to_string(),
        send_email::Task {
            email: user.email.clone(),
            name: user.real_name.clone(),
            subject: format!("Hi, {}", user.real_name),
            body: "This is a test email.".to_string(),
        },
    )?;
    Ok(Json(()))
}
