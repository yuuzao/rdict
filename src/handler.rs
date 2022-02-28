#![allow(dead_code, unused)]

use colored::Colorize;
use std::fmt;
use std::io::{self, Read};
use std::prelude;

pub mod youdao;

#[derive(Debug)]
pub enum Variety {
    US,
    UK,
}

#[derive(Debug, Default)]
pub struct PhoneticUri {
    uk: Option<String>,
    us: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Phonetic {
    us: Option<String>,
    uk: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct Explain {
    content: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct Example {
    sentence_eng: String,
    trans: String,
}

#[derive(Debug, Clone)]
pub struct Typo {
    pub guessing: Option<String>,
    pub meaning: Option<String>,
}

#[derive(Debug)]
pub struct VocabBody {
    phrase: String,
    phonetic: Option<Phonetic>,
    explains: Option<Vec<Explain>>,
    examples: Option<Vec<Example>>,
    typo: Option<Vec<Typo>>,
}

impl VocabBody {
    pub fn new(phrase: String) -> Self {
        VocabBody {
            phrase,
            phonetic: None,
            explains: None,
            examples: None,
            typo: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.phonetic.is_none()
            && self.explains.is_none()
            && self.examples.is_none()
            && self.typo.is_none()
    }

    // pub fn as_bytes(&self) ->
}

impl fmt::Display for VocabBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f);
        if self.is_empty() {
            writeln!(f, "{:>4}{}", " ", r#"¯\_(ツ)_/¯"#.truecolor(95, 175, 95));
            writeln!(f, "{:>4}{}", " ", "No result found".truecolor(95, 175, 95));
            return Ok(());
        }
        if let Some(p) = self.phonetic.clone() {
            writeln!(f, "{t:>4}", t = "音标".truecolor(255, 95, 175));
            let mut wp = |us_uk: String, ph: String| {
                write!(
                    f,
                    "{zh:>7}{lb}{phonetic}{rb}",
                    zh = ph.truecolor(0, 175, 175),
                    lb = "[".truecolor(188, 188, 188),
                    phonetic = us_uk.truecolor(95, 175, 95),
                    rb = "]".truecolor(188, 188, 188),
                );
            };

            if let Some(uk) = p.uk {
                wp(uk, "英".to_string())
            }
            if let Some(us) = p.us {
                wp(us, "美".to_string())
            }
        }
        writeln!(f);
        if let Some(exp) = &self.explains {
            writeln!(f, "{t:>4}", t = "释义".truecolor(255, 95, 175));
            for e in exp.iter() {
                if let Some(i) = e.content.clone().unwrap().split_once('.') {
                    writeln!(
                        f,
                        "{p:>6}{part}{dot}{zh}",
                        p = ' ',
                        part = i.0.trim().truecolor(0, 175, 175),
                        dot = ". ".truecolor(188, 188, 188),
                        zh = i.1.trim().truecolor(95, 175, 95),
                    );
                } else {
                    writeln!(
                        f,
                        "{p:>6}{dot}{zh}",
                        p = ' ',
                        dot = ">> ".truecolor(0, 175, 175),
                        zh = e.content.clone().unwrap().trim().truecolor(95, 175, 95)
                    );
                }
            }
        }
        if let Some(exa) = &self.examples {
            writeln!(f, "{:>4}", "例句".truecolor(255, 95, 175));
            for (i, e) in exa.clone().iter().enumerate() {
                write!(
                    f,
                    // align issue if index large than 10, but this will never happen
                    "{p:>6}{index}{dot}",
                    p = " ",
                    index = (i + 1).to_string().truecolor(0, 175, 175),
                    dot = ". ".truecolor(188, 188, 188),
                );

                let phrase = self.phrase.clone().to_lowercase();
                let mut sp = String::new();
                for v in e.sentence_eng.split(' ') {
                    let x = v.to_lowercase();
                    if x.starts_with(phrase.as_str()) || x.ends_with(phrase.as_str()) {
                        sp.push_str(v.truecolor(30, 250, 110).to_string().as_str());
                    } else {
                        sp.push_str(v.truecolor(95, 175, 95).to_string().as_str());
                    }
                    sp.push(' ');
                }
                writeln!(f, "{se}", se = sp);
                writeln!(f, "{p:>11}{st}", p = " ", st = e.trans.truecolor(0, 135, 0));
            }
        } else if let Some(typo) = &self.typo {
            writeln!(
                f,
                "{:>4}{phrase} {msg}",
                ' ',
                phrase = &self.phrase.truecolor(30, 250, 110),
                msg = "may be a typo, are you looking for:".truecolor(95, 175, 95)
            );
            for w in typo {
                if let Some(g) = &w.guessing {
                    writeln!(
                        f,
                        "{:>6}{} {word}",
                        ' ',
                        ">".truecolor(188, 188, 188),
                        word = g.truecolor(30, 250, 110)
                    );
                }
                if let Some(m) = &w.meaning {
                    if let Some(i) = m.split_once('.') {
                        writeln!(
                            f,
                            "{:>8}{}{dot} {m}",
                            ' ',
                            part = i.0.truecolor(0, 175, 175),
                            dot = ".".truecolor(188, 188, 188),
                            m = i.1.trim().truecolor(95, 175, 95)
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum QueryError {
    RequestError(Box<ureq::Error>),
    SerdeError(serde_json::Error),
    InputError(io::Error),
}

impl From<ureq::Error> for QueryError {
    fn from(err: ureq::Error) -> QueryError {
        QueryError::RequestError(Box::new(err))
    }
}

impl From<serde_json::Error> for QueryError {
    fn from(err: serde_json::Error) -> QueryError {
        QueryError::SerdeError(err)
    }
}
impl From<io::Error> for QueryError {
    fn from(err: io::Error) -> QueryError {
        QueryError::InputError(err)
    }
}

pub trait Query {
    fn query_meaning(&self, text: &str) -> Result<Vec<u8>, QueryError>;
    fn query_pronounce(&self, text: Option<&str>) -> Result<PhoneticUri, QueryError>;
}
