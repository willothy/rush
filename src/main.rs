use std::io::*;
use std::process::{Command, Stdio, Child};
use std::path::{self, PathBuf};
use std::env;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use substring::Substring;
use whoami;
use crossterm::{ExecutableCommand, terminal};

mod rush_path;
mod lib;

use rush_path::RushPath;

fn main() {
    init_shell();
    main_loop();
}

fn main_loop() {
    pyo3::prepare_freethreaded_python();
    let gil = Python::acquire_gil();
    let py: Python = gil.python();

    let mut input: String = String::new();
    let mut current_dir: RushPath = RushPath::from(env::current_dir().unwrap());
    let mut multiline: bool = false;

    loop {
        if multiline {
            input.clear();
        }

        print!("{} in {}\nrush on {} > ", whoami::username(), current_dir, whoami::hostname()); // Temporary Prompt
        let _ = stdout().flush();


        //stdin().read_line(&mut input).unwrap();
        match stdin().read_line(&mut input) {
            Ok(0) => continue,
            Ok(_) => {},
            Err(_) => continue
        };

        if input.trim().ends_with("\\") {
            multiline = true;
            continue;
        } else {
            multiline = false;
        }

        let mut commands = input
            .trim()
            .split(" | ")
            .peekable();
        //let mut previous_command = None;

        while let Some(command) = commands.next() {
            // Shadow input with trimmed input
            let input = command
                .trim();
                //.split_whitespace();
            let mut input_split = input.split_whitespace();
            let command = input_split.next().unwrap();
            let args = input_split;

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
                    let new_dir = path::Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&new_dir) {
                        eprintln!("{}", e);
                        env::set_current_dir("/").unwrap();
                    }
                    current_dir.set(match env::current_dir() {
                        Ok(dir) => dir,
                        Err(_) => PathBuf::from("/")
                    });
                    set_title(String::from(format!("rush {}", new_dir.to_str().unwrap())));
                    //previous_command = None;
                },
                "exit" => return,
                _ => {}
            }
            match input {
                code => {
                    let mut code_chars = code.chars().enumerate();
                    let mut code = String::from(code);
                    loop {
                        //println!("{}", code_chars);
                        if let Some((index, c)) = code_chars.next() {
                            if c == ':' {
                                code.insert_str(index + 2, "\n");
                                println!("got {} @ {}, code: {}", c, index, code);
                            }
                        } else if let None = code_chars.next() {
                            break;
                        }
                    }
                    /*items.iter().enumerate().for_each(|(i, x)| {
                        println!("Item {} = {}", i, x);
                    })*/
                    match run_py(py, code) {
                        Ok(result) => {
                            lib::write_raw(result.to_string().as_str()).unwrap();
                        },
                        Err(e) => {
                            eprintln!("{}\n", e);
                        }
                    }
                }
            }
        }

        /*if let Some(mut final_command) = previous_command {
            // wait until final command has finished, print if there's an error
            if let Err(e) = final_command.wait() {
                eprintln!("{}", e);
            }
        }*/
    }
}


fn run_py(py: Python, code: String) -> PyResult<&PyAny> {
    py.eval(code.as_str(), None, None)
}

fn init_shell() {
    let home = match home::home_dir() {
        Some(home_dir) => home_dir,
        None => PathBuf::from("/")
    };
    match env::set_current_dir(home) {
        Ok(_) => {
            //set_title(format!("rush {}", current_dir()));
        },
        Err(_) => {
            env::set_current_dir("/").unwrap();
            //set_title(format!("rush {}", current_dir()));
        }
    }
    clear();
}

fn set_title(title: String) {
    crossterm::execute!(stdout(), terminal::SetTitle(title)).unwrap();
}

// Clear shell using escape sequence
fn clear() {
    match stdout().execute(terminal::Clear(terminal::ClearType::All)) {
        Ok(_) => {},
        Err(e) => println!("ERROR: {}", e)
    }
}


