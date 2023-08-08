use url::Url;

pub fn extract_hostname(url: &str) -> Option<String> {
  let parsed_url = Url::parse(url);

  if let Ok(url) = parsed_url {
    if let Some(host) = url.host_str() {
      return Some(host.to_string());
    }
  } else if let Some(index) = url.find('@') {
    let rem = &url[index + 1..];
    if let Some(colon_pos) = rem.find(':') {
      return Some(rem[..colon_pos].to_string());
    }
  }

  None
}

pub fn extract_owner_and_repo(url: &str) -> Option<(String, String)> {
  if let Ok(url) = Url::parse(url) {
    if let Some(segments) = url.path_segments() {
      let mut segments_iter = segments.into_iter();
      if let Some(owner) = segments_iter.next() {
        if let Some(repo) = segments_iter.next() {
          return Some((owner.to_string(), repo.to_string()));
        }
      }
    }
  } else if let Some(index) = url.find(':') {
    let rem = &url[index + 1..];
    if let Some(slash_pos) = rem.find('/') {
      let owner = rem[..slash_pos].to_string();
      let repo = rem[slash_pos + 1..]
        .trim_end_matches(".git")
        .to_string();
      return Some((owner, repo));
    }
  }

  None
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
      assert_eq!(result.as_deref(), expected);
    }
  }

  #[test]
  fn test_extract_owner_and_repo() {
    let test_cases: Vec<(&str, Option<(&str, &str)>)> = vec![
      ("https://www.example.com/path/to/something", Some(("path", "to"))),
      ("git@github.com:owner/repo.git", Some(("owner", "repo"))),
      ("invalid", None),
      ("", None),
    ];

    for (input, expected) in test_cases {
      let result = extract_owner_and_repo(input);
      match (result, expected) {
        (Some((actual_owner, actual_repo)), Some((expected_owner, expected_repo))) => {
          assert_eq!(actual_owner, expected_owner);
          assert_eq!(actual_repo, expected_repo);
        }
        (None, None) => {}
        _ => panic!("Test failed for: {}", input),
      }
    }
  }
}
