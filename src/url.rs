use std::error::Error;

use url::Url;

pub fn extract_hostname(url: &str)
  -> Result<Option<String>, Box<dyn Error>>
{
  let parsed_url = Url::parse(url);

  if let Ok(url) = parsed_url {
    if let Some(host) = url.host_str() {
      return Ok(Some(host.to_string()));
    }
  } else if let Some(index) = url.find('@') {
    let rem = &url[index + 1..];
    if let Some(colon_pos) = rem.find(':') {
      return Ok(Some(rem[..colon_pos].to_string()));
    }
  }

  Ok(None)
}

pub fn extract_segments(url: &str)
  -> Result<Option<String>, Box<dyn Error>>
{
  fn extract_common(segments: Vec<&str>)
    -> Result<Option<String>, Box<dyn Error>>
    {
    let mut vec_segments: Vec<String> = segments.iter()
      .map(|&s| s.to_string())
      .collect();
    if !vec_segments.is_empty() {
      vec_segments.truncate(vec_segments.len() -1);
      Ok(Some(vec_segments.join("/")))
    } else {
      Ok(None)
    }
  }
  if let Ok(url) = Url::parse(url) {
    let segments: Vec<&str> = url.path_segments()
      .unwrap()
      .collect();
    return extract_common(segments);
  } else if let Some(index) = url.find(':') {
    // git@host:owner/subgroup/group.git
    let rem = &url[index + 1..];
    if let Some(dotgit) = rem.find(".git") {
      let segments: Vec<&str> = rem[..dotgit]
        .split("/")
        .collect();
      return extract_common(segments);
    }
  }

  Ok(None)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_extract_hostname() {
    let test_cases: Vec<(&str, Option<&str>)> = vec![
      ("https://www.example.com/path/to/something", Some("www.example.com")),
      ("ssh://username@hostname:22/path/to/resource", Some("hostname")),
      ("git@github.com:owner/repo.git", Some("github.com")),
      ("invalid", None),
      ("", None),
    ];

    for (input, expected) in test_cases {
      let result = extract_hostname(input);
      assert_eq!(result.unwrap().as_deref(), expected);
    }
  }

  #[test]
  fn test_extract_segments() {
    let test_cases: Vec<(&str, Option<&str>)> = vec![
      ("https://www.example.com/path/to/something", Some("path/to")),
      ("git@github.com:owner/repo.git", Some("owner")),
      ("git@gitlab.com:owner/subgroup/repo.git", Some("owner/subgroup")),
      ("git@gitlab.com:owner/some/deep/nested/repo.git", Some("owner/some/deep/nested")),
      ("invalid", None),
      ("", None),
    ];

    for (input, expected) in test_cases {
      let result = extract_segments(input);
      assert_eq!(result.unwrap().as_deref(), expected);
    }
  }
}
