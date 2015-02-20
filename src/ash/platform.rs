/**
 * Platform specific functions for ash
 */
use std::os;
use std::env;

#[cfg(windows)]
pub fn get_username() -> String {
  match env::var_os("USERNAME") {
    Some(e) => e,
    None => "?".to_string()
  }
}

#[cfg(unix)]
pub fn get_username() -> String {
  match os::getenv("USER") {
    Some(e) => e,
    None => "?".to_string()
  }
}

//I don't know how to get this yet
pub fn get_hostname() -> String {
  "hostname".to_string()
}
