

use Schema::(posts,users);

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}
