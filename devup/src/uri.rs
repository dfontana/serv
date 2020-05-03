use std::str::FromStr;

#[derive(Debug)]
pub struct URI {
  username: String,
  subdomain: String,
  domain: String,
  tld: String,
}

impl URI {
  pub fn to_string(&self) -> String {
    let mut bldr = self.username.to_owned();
    if !bldr.is_empty() {
      bldr.push('@');
    }
    if !self.subdomain.is_empty() {
      bldr.push_str(&self.subdomain);
      bldr.push('.');
    }
    bldr.push_str(&self.domain);
    bldr.push('.');
    bldr.push_str(&self.tld);
    return bldr;
  }
}

type ParseError = &'static str;

impl FromStr for URI {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, ParseError> {
    let user_host: Vec<&str> = s.trim().split('@').collect();
    match user_host.len() {
      1 => parse_username("", user_host[0]),
      2 => parse_username(user_host[0], user_host[1]),
      _ => Err("Too many '@' signs"),
    }
  }
}

fn parse_username(username: &str, domain: &str) -> Result<URI, ParseError> {
  let parts: Vec<&str> = domain.split('.').collect();
  match parts.len() {
    2 => parse_domain(username, "", parts[0], parts[1]),
    3 => parse_domain(username, parts[0], parts[1], parts[2]),
    _ => Err("Too many domain parts"),
  }
}

fn parse_domain(
  username: &str,
  subdomain: &str,
  domain: &str,
  tld: &str,
) -> Result<URI, ParseError> {
  Ok(URI {
    username: username.to_string(),
    subdomain: subdomain.to_string(),
    domain: domain.to_string(),
    tld: tld.to_string(),
  })
}
