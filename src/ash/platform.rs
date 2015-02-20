/**
 * Platform specific functions for ash
 */
use std::env;

#[cfg(windows)]
pub fn get_username() -> String {
  match env::var_os("USERNAME") {
    Some(e) => e.into_string().unwrap(),
    None => "?".to_string()
  }
}

#[cfg(unix)]
pub fn get_username() -> String {
  match env::var_os("USER") {
    Some(e) => e.into_string().unwrap(),
    None => "?".to_string()
  }
}

//I don't know how to get this yet
pub fn get_hostname() -> String {
  "hostname".to_string()
}
