//! 数据库相关功能
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use dotenv::dotenv;
use std::env;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

/// 建立数据库连接池, 通过`server`模块，保存到actix的data里。
pub fn connect() -> MysqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init(&database_url).expect("Error")
}


// /// 建立数据库连接
// pub fn establish_connection() -> MysqlConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }
