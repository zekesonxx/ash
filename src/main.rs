use std::io;
use std::io::fs;
use std::os;
use std::io::process::{Command};
use std::path::Path;

mod ash;


fn main() {
  println!("ash: A shell");
  println!("Incredibly in beta");
  println!("May eat your left shoes.");

  let mut cwd = os::getcwd().unwrap();

  loop {
    print!("{}", ash::format::format(&cwd));
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
          cwd = os::homedir().unwrap();
          continue;
        }
        let path: Option<Path> = if args[0].starts_with("~") { // ~/projects
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
                if stat.kind == io::FileType::Directory {
                  cwd = new
                }
              },
              Err(e) => println!("No such file or directory: \"{}\"", args[0])
            }
          }
          None => println!("Failed to locate path")
        }
      }
      "exit" => {
        println!("Goodbye!");
        break;
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
