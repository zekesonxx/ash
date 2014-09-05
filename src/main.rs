
use std::io;
use std::io::fs;
use std::os;
use std::io::process::{Command};
use std::path::Path;

#[cfg(windows)]
fn get_username() -> String {
  match os::getenv("USERNAME") {
    Some(e) => e,
    None => "?".to_string()
  }
}

#[cfg(unix)]
fn get_username() -> String {
  match os::getenv("USER") {
    Some(e) => e,
    None => "?".to_string()
  }
}

//I don't know how to get this yet
fn get_hostname() -> String {
  "hostname".to_string()
}

fn format_cwd(cwd: &Path, home: &Path) -> String {
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


fn main() {
  println!("ash: A shell");
  println!("Incredibly in beta");
  println!("May eat your left shoes.");
  
  //setup the persistant variables
  let mut cwd = os::getcwd();
  let home = match os::homedir() {
    Some(e) => e,
    None => Path::new("/")
  };
  let hoststring = format!("{user}@{host}",
    user=get_username(),
    host=get_hostname()
  );
  let symbol = if get_username() == "root".to_string() {
    "#"
  } else {
    "$"
  };

  loop {
    print!("{hoststring} {cwd} {symbol} ", 
      hoststring=hoststring,
      cwd=format_cwd(&cwd, &home), 
      symbol=symbol
    );
    let rawinput = io::stdin().read_line().ok().expect("Error Occured");
    let input = rawinput.as_slice().trim();
    if input == "" { continue } //skip blank enters
    let opts: Vec<&str> = input.split_str(" ").collect();
    let (cmd, args) = (opts[0], opts.slice(1, opts.len()));
    match cmd {
      "cd" => {
        //set the current directory
        if args[0] == "~" {
          //go home
          cwd = home.clone();
          continue;
        }
        let path: Option<Path> = if args[0].starts_with("~") { // ~/projects
          //this is intentionally NOT using the `home` variable
          match os::homedir() {
            Some(e) => {
              let dir = e.join(args[0].slice_from(2)); //[~/]
              Some(dir)
            }, //hacky but whatever
            None => Path::new_opt(args[0])
          }
        } else if args[0].starts_with(".") || !args[0].starts_with("/") { // ./bin or ../ash || cd src
          Some(cwd.join(args[0]))
        } else {
          Path::new_opt(args[0])
        };
        match path {
          Some(new) => {
            match fs::stat(&new) {
              Ok(stat) => {
                match stat.kind {
                  TypeDirectory => {cwd = new} // why is this so unhappy
                } 
              },
              Err(e) => println!("No such file or directory: \"{}\"", args[0])
            }
          }
          None => println!("Failed to locate path")
        }
      }
      _ => {
        let process = Command::new(cmd).cwd(&cwd).args(args).output();

        match process {
          Ok(output) => {
            println!("{}", String::from_utf8_lossy(output.output.as_slice()));

          },
          Err(e) => {
            println!("Error Occured: {}", e);
          },
        };
      }
    }
  }
}
