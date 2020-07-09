#[derive(Debug)]
pub struct User {
  pub id: Option<i16>,
  pub name: String,
  pub age: i8,
  pub email: String,
  pub password: String,
}