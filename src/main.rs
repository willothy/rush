use std::io::*;
use std::process::{Command, Stdio};

mod lib;

fn main() {
    init_shell();
    main_loop();
}

fn main_loop() {
    let username = lib::get_user();
    let mut command: String = String::with_capacity(1024);
    loop {
        command.clear();
        print!("{}\nrush > ", username);
        let _ = stdout().flush();
        stdin().read_line(&mut command).unwrap();


        let process = Command::new(command.trim())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match process {
            Ok(v) => {                
                match v.wait_with_output() {
                    Ok(data) => {
                        match String::from_utf8(data.stdout) {
                            Ok(output) => println!("{}", output),
                            Err(e) => println!("Error: {}", e)
                        }
                    },
                    Err(e) => println!("Error: {}", e)
                }
            },
            Err(e) => println!("Error: {}", e)
        }
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


