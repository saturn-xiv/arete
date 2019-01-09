pub mod migration;
pub mod schema;

pub type Connection = diesel::pg::PgConnection;
pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>;
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

#[database("postgresql")]
pub struct Database(Connection);
