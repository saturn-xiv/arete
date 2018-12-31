pub type Connection = r2d2_redis::redis::Connection;
pub type Pool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;
pub type PooledConnection = r2d2_redis::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;
