use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self,FromRequest};
use rocket::{Request,State,Outcome};
use diesel::pg::PgConnection;
//use diesel::r2d2::{ConnectionManager};
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;


pub type pg_conn = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> pg_conn {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    pg_conn::new(manager).expect("Db pool")
}

fn database_url() -> String{
  //  env::var("DATABASE_URL").expect("Database url should be set")
    return "postgres://postgres:RamKrishna123@localhost/myblogsrs".to_string()
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error>{

        let pool = request.guard::<State<pg_conn>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable,())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}





