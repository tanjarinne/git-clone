use std::env;

mod url;

fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(host) = url::extract_hostname(&args[1]) {
    println!("Git host: {}", host);
  }
}
