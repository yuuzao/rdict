use std::{env, path::PathBuf};

use colored::Colorize;

use crate::result::Result;

const DB_NAME: &str = "rdict";
pub enum ColorfulRole {
    Title,
    Index,
    Dot,
    Content,
    Emphasis,
    Logo,
    Wip,
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
            ColorfulRole::Wip => (214, 158, 19),
        }
    }
}

pub trait Style
where
    Self: ToString,
{
    fn coloring(&self, role: ColorfulRole) -> String {
        let rgb: (u8, u8, u8) = role.into();
        Colorize::truecolor(self.to_string().as_str(), rgb.0, rgb.1, rgb.2).to_string()
    }

    fn align_right(&self, width: usize) -> String {
        let s = self.to_string();
        let n = if width > s.len() { width - s.len() } else { 0 };
        let pad = " ".repeat(n);
        format!("{}{}", pad, s)
    }

    fn align_left(&self, width: usize) -> String {
        let s = self.to_string();
        let n = if width > s.len() { width - s.len() } else { 0 };
        let pad = " ".repeat(n);
        format!("{}{}", s, pad)
    }
}

impl<T> Style for T where T: ToString {}
