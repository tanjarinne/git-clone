use std::env;

mod url;
pub mod git;

fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(host) = url::extract_hostname(&args[1]) {
    println!("Git host: {}", host);
  }

  if let Ok(Some(root_path)) = git::root_dir() {
    println!("Root path: {}", root_path);
  }
}
