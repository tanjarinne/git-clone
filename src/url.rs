use url::Url;

pub fn extract_hostname(url: &str) -> Option<String> {
  if let Ok(url) = Url::parse(url) {
    if let Some(host) = url.host_str() {
      return Some(host.to_string());
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
}
