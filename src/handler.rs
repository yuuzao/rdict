#![allow(dead_code, unused)]

use colored::Colorize;
use reqwest;
use serde_json;
use std::fmt;
use std::io;
use url;

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
    uk: String,
    us: String,
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
#[derive(Debug)]
pub struct VocabBody {
    phrase: Option<String>,
    phonetic: Phonetic,
    explains: Vec<Explain>,
    examples: Vec<Example>,
}

impl fmt::Display for VocabBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{ld}\n{zh:>5}{lb}{phonetic}{rb}",
            ld = "音标".truecolor(255, 95, 175),
            zh = "英".truecolor(0, 175, 175),
            lb = "[".truecolor(188, 188, 188),
            phonetic = self.phonetic.uk.clone().unwrap().truecolor(95, 175, 95),
            rb = "]".truecolor(188, 188, 188),
        );
        write!(
            f,
            "{zh:>5}{lb}{phonetic}{rb}\n",
            zh = "美".truecolor(0, 175, 175),
            lb = "[".truecolor(188, 188, 188),
            phonetic = self.phonetic.us.clone().unwrap().truecolor(95, 175, 95),
            rb = "]".truecolor(188, 188, 188),
        );
        write!(f, "\n");
        write!(f, "{t}\n", t = "释义".truecolor(255, 95, 175));
        for e in self.explains.iter() {
            for i in e.content.clone().unwrap().split_once(".") {
                write!(
                    f,
                    "{p:>4}{part:>3}{dot}{zh}\n",
                    p = " ",
                    dot = ".".truecolor(188, 188, 188),
                    part = i.0.truecolor(0, 175, 175),
                    zh = i.1.truecolor(95, 175, 95),
                );
            }
        }
        write!(f, "\n");
        write!(f, "{}\n", "例句".truecolor(255, 95, 175));
        for (i, e) in self.examples.iter().enumerate() {
            write!(
                f,
                // align issue if index large than 10, but this will never happen
                "{p:>4}{index}{dot}",
                p = " ",
                index = (i + 1).to_string().truecolor(0, 175, 175),
                dot = ". ".truecolor(188, 188, 188),
            );

            let phrase = self.phrase.clone().unwrap().to_lowercase();
            let mut sp = String::new();
            for v in e.sentence_eng.split(" ") {
                let x = v.to_lowercase();
                if x.starts_with(phrase.as_str()) || x.ends_with(phrase.as_str()) {
                    sp.push_str(v.truecolor(30, 250, 110).to_string().as_str());
                } else {
                    sp.push_str(v.truecolor(95, 175, 95).to_string().as_str());
                }
                sp.push_str(" ");
            }
            write!(f, "{se}\n", se = sp);

            write!(
                f,
                "{p:>7}{st}\n",
                p = " ",
                st = e.trans.truecolor(0, 135, 0)
            );
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
