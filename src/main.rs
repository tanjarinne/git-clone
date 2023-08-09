use std::{env, process::Command, path::PathBuf};

mod url;
mod git;

fn mkdir(path: &str) -> Result<(), std::io::Error> {
  std::fs::create_dir_all(path)?;
  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();

  let root_path = git::root_dir()?
    .expect("Couldn't resolve root path");
  let host = url::extract_hostname(&args[1])?
    .expect(&format!("Couldn't resolve the hostname from {}", &args[1]));
  let path = url::extract_segments(&args[1])?
    .expect(&format!("Couldn't resolve segments from {}", &args[1]));

  let mut new_dir = PathBuf::new();
  new_dir.push(&root_path);
  new_dir.push(&host);
  new_dir.push(&path);

  let asfsadf = format!("{}", &new_dir.display());
  let new_dir_expanded = shellexpand::tilde(&asfsadf);
  if let Err(err) = mkdir(&new_dir_expanded) {
    eprintln!("Error creating directory structure {}", err);

  }

  Command::new("git")
    .arg("clone")
    .current_dir(new_dir_expanded.to_string())
    .args(&args[1..])
    .spawn()?
    .wait()?;

  Ok(())
}
