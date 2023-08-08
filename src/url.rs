use url::Url;

pub fn extract_hostname(url: &str) -> Option<String> {
  if let Ok(url) = Url::parse(url) {
    if let Some(host) = url.host_str() {
      return Some(host.to_string());
    }
  }

  None
}
