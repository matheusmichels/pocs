#[path = "controllers/user_controller.rs"] mod user_controller;

const HELLO_WORLD: &str = "Hello world in Rust";

fn main() {
  println!("{}", HELLO_WORLD);

  let mut user = user_controller::random_user();
  println!("Generating random user -> {:?}", user);
  user = user_controller::create(user);
  println!("Creating user -> {:?}", user);

  let mut vec = Vec::new();
  vec.push(1);
  vec.push(5);
  vec.push(9);

  println!("Vector {:?}", vec);

  for x in vec {
    println!("Iterating {}", x);
  }

  let arr = [1, 2, 4, 5];
  let [first, _, _, last] = arr;

  println!("Destructuring -> first {} and last {}", first, last);
}

