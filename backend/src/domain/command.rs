#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: Option<String>
}

#[derive(Debug)]
pub struct Message {
    pub id: i64,
    pub user: Option<User>,
    pub text: String
}