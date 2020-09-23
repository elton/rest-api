use diesel::prelude::*;
/// 数据库相关功能
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, PoolError},
};
use dotenv::dotenv;
use std::env;

/// 建立数据库连接
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
