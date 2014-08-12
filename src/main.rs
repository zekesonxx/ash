
use std::io;
use std::os;
use std::io::process::{Command, Process};
use std::path::Path;

fn main() {
  println!("ash Burning Fucking Shit shell")
  let mut cwd = os::getcwd();
  loop {
    print!("{} $ ", cwd.display());
    let rawinput = io::stdin().read_line().ok().expect("Error Occured");
    let input = rawinput.as_slice().trim();
    match input {
      "~" => {
        //temp, go home.
        cwd = match os::homedir() {
          Some(e) => Path::new(e),
          None => Path::new("/")
        }
      }
      "" => {} //no input
      _ => {
        let opts: Vec<&str> = input.split_str(" ").collect();
        match Command::new(opts[0]).cwd(&cwd).args(opts.slice(1, opts.len())).output() {
          Ok(output) => {
            println!("[{}] {}",output.status, String::from_utf8_lossy(output.output.as_slice()));

          },
          Err(e) => {
            println!("Error Occured: {}", e);
          },
        };
      }
    }
  }
}
