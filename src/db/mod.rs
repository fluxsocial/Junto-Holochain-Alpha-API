use diesel::r2d2::{self, PooledConnection, ConnectionManager};
use diesel::pg::PgConnection;

pub mod models;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;