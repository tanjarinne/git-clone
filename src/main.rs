use std::env;

mod url;
mod git;

fn mkdir(path: &str) -> Result<(), std::io::Error> {
  std::fs::create_dir_all(path)?;
  Ok(())
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let root_path = git::root_dir()
    .unwrap()
    .expect("Couldn't resolve root path");
  let host = url::extract_hostname(&args[1]).unwrap();
  let path = url::extract_segments(&args[1]).unwrap();

  let new_dir = format!("{}/{}/{}", root_path, host, path);
  let new_dir_expanded = shellexpand::tilde(&new_dir);
  if let Err(err) = mkdir(&new_dir_expanded) {
    eprintln!("Error creating directory structure {}", err);
    return;
  }
  print!("{}/{}/{}", root_path, host, path);
}
