use std::io::*;
use std::process::{Command, Stdio};
use std::path;
use std::env;

mod lib;

fn main() {
    init_shell();
    main_loop();
}

fn main_loop() {
    let username = lib::get_user();
    let pwd = match lib::find_var("PWD") {
        Some(dir) => dir,
        None => {
            eprintln!("Error finding pwd.");
            String::from("/?")
        }
    };
    let mut input: String = String::with_capacity(1024);
    loop {
        input.clear();
        
        print!("{} in {}\nrush > ", username, pwd);
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();

        // Shadow input with trimmed input
        let mut input = input
            .trim()
            .split_whitespace();
        let command = input.next().unwrap();
        let args = input;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args
                    .peekable()
                    .peek()
                    .map_or("/", |x| *x);
                let root = path::Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                run_command(command, args);
            }
        }
    }
}

fn run_command(command: &str, args: std::str::SplitWhitespace) {
    let process = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match process {
        Ok(v) => {
            match v.wait_with_output() {
                Ok(data) => {
                    if let Ok(output) = String::from_utf8(data.stdout) {
                        println!("{}", output)
                    }
                },
                Err(e) => eprintln!("Error: {}", e)
            }
        },
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn init_shell() {
    clear();
}

// Clear shell using escape sequence
fn clear() {
    match lib::write_raw("\x1b[H\x1b[J") {
        Ok(_) => {},
        Err(e) => println!("ERROR: {}", e)
    }
}


