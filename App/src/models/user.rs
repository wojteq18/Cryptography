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
    //sprawdź czy użytkownik istnieje
    pub fn user_exist(conn: &mut PooledConn, login: &str) -> bool {
        let exists: Option<String> = conn.exec_first(
            "SELECT login FROM users WHERE login = :login",
            params! {
                "login" => login,
            }
        ).unwrap_or(None);
        exists.is_some() //zwróc true, jeśli użytkownik istnieje
    }

    //konstruktor nowego użytkownika
    pub fn new(login: &str, password: &str, p: &i64, q: &i64, n: &i64, fi_n: &i64) -> Self {
        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => panic!("Failed to hash password"),
        };

        User {
            user_id: 0,
            login: login.to_owned(),
            password: hashed_password,
            p: *p,
            q: *q,
            n: *n,
            fi_n: *fi_n,
        }
    }

    //dodanie użytkownika jeśli nie istnieje
    pub fn add_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        //TODO
    }
}