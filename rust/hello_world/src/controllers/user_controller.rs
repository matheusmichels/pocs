use rand::Rng;

#[path = "../models/user.rs"] mod user;

pub fn random_user() -> user::User {
  let user = user::User {
    id: None,
    name: String::from("Matheus Michels"),
    age: 12,
    email: String::from("hi@mm.com"),
    password: String::from("123456"),
  };

  return user;
}

pub fn create(body: user::User) -> user::User {
  let mut created_user = body;

  let mut rng = rand::thread_rng();
  created_user.id = Some(rng.gen_range(0, 100));

  return created_user;
}