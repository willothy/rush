use std::path::PathBuf;
use std::fmt::{ Display, Formatter, Result };

pub struct RushPath(PathBuf);

impl RushPath {
    pub fn new() -> Self {
        RushPath(PathBuf::new())
    }

    pub fn from(p: PathBuf) -> Self {
        RushPath(p)
    }

    pub fn set(&mut self, new_path: PathBuf) {
        self.0 = new_path;
    }

    pub fn to_str(& self) -> Option<&str> {
        self.0.to_str()
    }
}

impl Display for RushPath {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self.0.to_str() {
            Some(path) => path,
            None => "/?"
        })
    }
}