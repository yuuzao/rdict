use colored::Colorize;
use std::{env, fs, io, path::PathBuf};

//TODO: refactory
pub fn colorize(s: &str, rgb: (u8, u8, u8)) -> String {
    s.truecolor(rgb.0, rgb.1, rgb.2).to_string()
}

const DB_NAME: &str = "rdict";

pub fn open_db() -> io::Result<sled::Db> {
    let path = match set_path() {
        Err(e) => return Err(e),
        Ok(p) => p,
    };

    match path.exists() {
        false => {
            if let Err(e) = fs::create_dir(&path) {
                return Err(e);
            }
        }
        true => {
            if !path.is_dir() {
                return Err(io::Error::new(io::ErrorKind::Other, "not a directory"));
            }
        }
    }

    match sled::open(path) {
        Ok(db) => Ok(db),
        Err(e) => Err(e.into()),
    }
}

fn set_path() -> io::Result<PathBuf> {
    let mut p = match dirs::data_dir() {
        Some(v) => v,
        None => env::current_dir()?,
    };
    p.push(DB_NAME);
    Ok(p)
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_db() {
        let db = open_db();
        assert!(db.is_ok());
    }

    #[test]
    fn test_set_path() {
        let p = set_path();
        assert!(p.is_ok());
        let p = p.unwrap();

        if cfg!(unix) {
            assert!(p.ends_with(".local/share/rdict"))
        } else if cfg!(windows) {
            assert!(p.ends_with(r#"\AppData\Local\rdict"#))
        } else if cfg!(macos) {
            assert!(p.ends_with("Library/Caches/rdict"))
        }
    }
}
