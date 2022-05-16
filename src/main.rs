use std::io::*;
use std::process::{Command, Stdio, Child};
use std::path::{self, PathBuf};
use std::env;
use whoami;

mod tokenizer;
mod parser;
mod lib;

fn main() {
    init_shell();
    main_loop();
}

fn main_loop() {
    //let pwd;
    let mut input: String = String::new();

    loop {
        input.clear();
        print!("{} in {}\nrush on {} > ", whoami::username(), env::current_dir().unwrap().to_str().unwrap(), whoami::hostname());
        let _ = stdout().flush();

        stdin().read_line(&mut input).unwrap();

        let mut commands = input
            .trim()
            .split(" | ")
            .peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            // Shadow input with trimmed input
            let mut input = command
                .trim()
                .split_whitespace();
            let command = input.next().unwrap();
            let args = input;

            match command {
                "cd" => {
                    // default to '~/' as new directory if one was not provided
                    // default to '/' if home dir doesn't exist
                    let home = match home::home_dir() {
                        Some(home_dir) => home_dir,
                        None => PathBuf::from("/")
                    };
                    let new_dir = args
                        .peekable()
                        .peek()
                        .map_or(home.to_str().unwrap_or("/"), |x| *x);
                    let root = path::Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let (stdout, stderr) = if commands.peek().is_some() {
                        (Stdio::piped(), Stdio::piped())
                    } else {
                        (Stdio::inherit(), Stdio::inherit())
                    };

                    let process = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .stderr(stderr)
                        .spawn();

                    match process {
                        Ok(output) => {
                            previous_command = Some(output)
                        },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("Error: {}", e)
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // wait until final command has finished, print if there's an error
            if let Err(e) = final_command.wait() {
                eprintln!("{}", e);
            }
        }
    }
}

fn run_command(command: &str, args: std::str::SplitWhitespace) {
    //
}

fn init_shell() {
    let home = match home::home_dir() {
        Some(home_dir) => home_dir,
        None => PathBuf::from("/")
    };
    match env::set_current_dir(home) {
        Ok(_) => {},
        Err(_) => env::set_current_dir("/").unwrap()
    }
    clear();
}

// Clear shell using escape sequence
fn clear() {
    match lib::write_raw("\x1b[H\x1b[J") {
        Ok(_) => {},
        Err(e) => println!("ERROR: {}", e)
    }
}


