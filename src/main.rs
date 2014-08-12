
use std::io;
use std::io::fs;
use std::os;
use std::io::process::{Command};
use std::path::Path;


fn main() {
  println!("ash Burning Fucking Shit shell")
  let mut cwd = os::getcwd();
  loop {
    print!("{} $ ", cwd.display());
    let rawinput = io::stdin().read_line().ok().expect("Error Occured");
    let input = rawinput.as_slice().trim();
    if input == "" { continue } //skip blank enters
    let opts: Vec<&str> = input.split_str(" ").collect();
    let (cmd, args) = (opts[0], opts.slice(1, opts.len()));
    match cmd {
      "cd" => {
        //set the current directory
        //only works absolutely atm
        if args[0] == "~" {
          //go home
          cwd = match os::homedir() {
            Some(e) => Path::new(e),
            None => Path::new("/")
          };
          continue;
        }
        let path: Option<Path> = if args[0].starts_with("~") {
          match os::homedir() {
            Some(e) => {
              let dir = e.join(args[0].slice_from(2)); //[~/]
              println!("{}", dir.display());
              Some(dir)
            }, //hacky but whatever
            None => Path::new_opt(args[0])
          }
        } else if args[0].starts_with(".") { //relative dir
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
            println!("[{}] {}", output.status, String::from_utf8_lossy(output.output.as_slice()));

          },
          Err(e) => {
            println!("Error Occured: {}", e);
          },
        };
      }
    }
  }
}
