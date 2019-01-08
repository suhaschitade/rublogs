use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::posts;
use chrono::prelude::*;
use diesel::pg::data_types::PgDate;
#[table_name="posts"]
#[derive(Serialize, Deserialize,Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub active: bool,
    pub image_path: String,
    pub sub_title: String
}

impl Post {
    pub fn read(connection: &PgConnection) -> Vec<Post>{
        posts::table.order(posts::id.asc()).load::<Post>(connection).unwrap()
    }

    pub fn get_details(key: i32, connection: &PgConnection) -> Vec<Post>{
        posts::table.find(key).load::<Post>(connection).unwrap()
    }
}

