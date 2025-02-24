use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: i32,
    pub login: String,
    pub password: String,
    pub p: i64,
    pub q: i64,
    pub n: i64,
    pub fi_n: i64,
}

impl User {

}