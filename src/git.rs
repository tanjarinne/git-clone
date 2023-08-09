use git2::Config;

const CLONE_ROOT_KEY: &str = "tanjarinne.git-clone.root-path";

fn read_from_gitconfig(key: &str) -> Result<Option<String>, git2::Error> {
    let config = Config::open_default()?;
    config.get_string(key).map(Some).or(Ok(None))
}

pub fn root_dir() -> Result<Option<String>, git2::Error> {
    read_from_gitconfig(CLONE_ROOT_KEY)
}
