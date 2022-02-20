#![allow(dead_code, unused)]

use colored::Colorize;
use std::fmt;
use std::io;

pub mod youdao;

#[derive(Debug)]
pub enum Variety {
    US,
    UK,
}
// TODO web_trans
// TODO meta
// TODO more examples

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
}

impl fmt::Display for VocabBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "");
        if let Some(p) = self.phonetic.clone() {
            writeln!(f, "{t:>4}", t = "音标".truecolor(255, 95, 175));
            if let Some(uk) = p.uk {
                write!(
                    f,
                    "{zh:>7}{lb}{phonetic}{rb}",
                    zh = "英".truecolor(0, 175, 175),
                    lb = "[".truecolor(188, 188, 188),
                    phonetic = self
                        .phonetic
                        .clone()
                        .unwrap()
                        .uk
                        .unwrap()
                        .truecolor(95, 175, 95),
                    rb = "]".truecolor(188, 188, 188),
                );
            }
            if let Some(us) = p.us {
                write!(
                    f,
                    "{zh:>4}{lb}{phonetic}{rb}",
                    zh = "美".truecolor(0, 175, 175),
                    lb = "[".truecolor(188, 188, 188),
                    phonetic = self
                        .phonetic
                        .clone()
                        .unwrap()
                        .us
                        .unwrap()
                        .truecolor(95, 175, 95),
                    rb = "]".truecolor(188, 188, 188),
                );
            }
        }
        writeln!(f, "");
        if self.explains.is_some() {
            writeln!(f, "{t:>4}", t = "释义".truecolor(255, 95, 175));
            for e in self.explains.clone().unwrap().iter() {
                if let Some(i) = e.content.clone().unwrap().split_once('.') {
                    writeln!(
                        f,
                        "{p:>6}{part}{dot} {zh}",
                        p = ' ',
                        dot = ".".truecolor(188, 188, 188),
                        part = i.0.trim().truecolor(0, 175, 175),
                        zh = i.1.trim().truecolor(95, 175, 95),
                    );
                } else {
                    writeln!(
                        f,
                        "{p:>6}{zh}",
                        p = ' ',
                        zh = e.content.clone().unwrap().trim().truecolor(95, 175, 95)
                    );
                }
            }
        }
        if self.examples.is_some() {
            writeln!(f, "{:>4}", "例句".truecolor(255, 95, 175));
            for (i, e) in self.examples.clone().unwrap().iter().enumerate() {
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
        } else if self.typo.is_some() {
            writeln!(
                f,
                "{:>4}{phrase} {msg}",
                ' ',
                phrase = self.phrase.clone().truecolor(30, 250, 110),
                msg = "may be a typo, are you looking for:".truecolor(95, 175, 95)
            );
            for w in self.typo.clone().unwrap() {
                if let Some(g) = w.guessing {
                    writeln!(
                        f,
                        "{:>6}{} {word}",
                        ' ',
                        ">".truecolor(188, 188, 188),
                        word = g.truecolor(30, 250, 110)
                    );
                }
                if let Some(m) = w.meaning {
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
    HttpError(reqwest::Error),
    UrlError(url::ParseError),
    SerdeError(serde_json::Error),
    InputError(io::Error),
}

impl From<reqwest::Error> for QueryError {
    fn from(err: reqwest::Error) -> QueryError {
        QueryError::HttpError(err)
    }
}

impl From<url::ParseError> for QueryError {
    fn from(err: url::ParseError) -> QueryError {
        QueryError::UrlError(err)
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
    fn query_meaning(&self, text: Option<&str>) -> Result<VocabBody, QueryError>;
    fn query_pronounce(&self, text: Option<&str>) -> Result<PhoneticUri, QueryError>;
}
