use std::env;
use std::io::*;

pub fn get_user() -> String {
    match find_var("USER") {
        Some(username) => username,
        None => {
            println!("Error: Can't resolve username.");
            String::from("unknown")
        }
    }
}

pub fn write_raw(data: &str) -> Result<()> {
    let mut out = BufWriter::new(stdout());

    match out.write(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn find_var(name: &str) -> Option<String> {
    let mut vars = env::vars().enumerate();
    loop {
        match vars.next() {
            Some(data) => {
                match data.1.0.as_str() {
                    _name if _name == name => {
                        return Some(String::from(data.1.1))
                    },
                    _ => continue
                }
            },
            None => return None
        }
    }
}