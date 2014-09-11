/**
 * Formatting the prompt
 */

use std::os;
use ash::platform;


pub fn format_cwd(cwd: &Path, home: &Path) -> String {
  match cwd.as_str() {
    Some(e) => {
      if home.is_ancestor_of(cwd) {
        format!("~{}", e.slice_from(home.as_vec().len()))
      } else {
        e.to_string()
      }
    },
    None => "?".to_string()
  }
}

pub fn format(cwd: &Path) -> String {
  let home = match os::homedir() {
    Some(e) => e,
    None => Path::new("/")
  };
  let hoststring = format!("{user}@{host}",
    user=platform::get_username(),
    host=platform::get_hostname()
  );
  let symbol = if platform::get_username() == "root".to_string() {
    "#"
  } else {
    "$"
  };
  format!("{hoststring} {cwd} {symbol} ",
    hoststring=hoststring,
    cwd=format_cwd(cwd, &home),
    symbol=symbol
  )
}