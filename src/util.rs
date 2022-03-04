use colored::Colorize;
use std::{env, path::PathBuf};

use crate::result::Result;

const DB_NAME: &str = "rdict";
pub enum ColorfulRole {
    Title,
    Index,
    Dot,
    Content,
    Emphasis,
    Logo,
    Other,
}

pub fn open_db() -> Result<sled::Db> {
    let path = match set_path() {
        Err(e) => return Err(e),
        Ok(p) => p,
    };

    return sled::open(path).map_err(Into::into);

    fn set_path() -> Result<PathBuf> {
        let mut p = match dirs::data_dir() {
            Some(v) => v,
            None => env::current_dir()?,
        };
        p.push(DB_NAME);
        Ok(p)
    }
}

// let's RGB

impl From<ColorfulRole> for (u8, u8, u8) {
    fn from(role: ColorfulRole) -> Self {
        match role {
            ColorfulRole::Title => (255, 95, 175),
            ColorfulRole::Index => (2, 169, 170),
            ColorfulRole::Dot => (188, 188, 188),
            ColorfulRole::Content => (95, 175, 95),
            ColorfulRole::Emphasis => (30, 250, 110),
            ColorfulRole::Logo => (0, 221, 192),
            ColorfulRole::Other => (0, 134, 1),
        }
    }
}

pub fn coloring<'a, T: Into<&'a str>>(s: T, role: ColorfulRole) -> String {
    let rgb: (u8, u8, u8) = role.into();
    s.into().truecolor(rgb.0, rgb.1, rgb.2).to_string()
}
