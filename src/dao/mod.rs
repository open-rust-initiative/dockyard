pub mod config_layers;
pub mod images;
pub mod layers;
pub mod library;
pub mod user;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;

pub type DatabaseConnect = PooledConnection<ConnectionManager<MysqlConnection>>;
