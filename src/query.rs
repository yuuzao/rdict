use std::{fmt, ops::Deref, sync::mpsc, thread, time};

use indicatif::{ProgressBar, ProgressStyle};

use crate::handler::{youdao, Query, VocabBody};
use crate::meta;
use crate::result::Result;
use crate::util;

#[derive(Debug, PartialEq, Eq)]
pub enum Engines {
    Youdao,
    Bing,
}

pub struct QueryTarget {
    pub engine: Engines,
    pub phrase: String,
    pub vocabulary: Option<VocabBody>,
    raw: Option<Vec<u8>>,
}

pub struct History(Vec<String>);

impl From<String> for Engines {
    fn from(eng: String) -> Self {
        match eng.as_str() {
            "bing" => Engines::Bing,
            _ => Engines::Youdao,
        }
    }
}

impl QueryTarget {
    pub fn new(phrase: String) -> Self {
        QueryTarget {
            engine: Engines::Youdao,
            phrase,
            vocabulary: None,
            raw: None,
        }
    }

    pub fn query(&mut self) -> Result<()> {
        let target = match self.engine {
            Engines::Bing => {
                meta::wip();
                std::process::exit(0)
            }
            _ => youdao::Youdao::new(self.phrase.as_str()),
        };

        self.raw = if let Some(cache) = self.from_cache()? {
            Some(cache)
        } else {
            target.query_meaning(&self.phrase).ok()
        };

        Ok(())
    }

    pub fn query_with_pb(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        if self.query().is_ok() {
            tx.send(1).unwrap();
        }

        // let's spin
        let jh = thread::spawn(move || {
            println!();
            let bar = bar();

            // just spin for 100 times before continue.
            for _ in 0..100 {
                bar.inc(1);
                thread::sleep(time::Duration::from_millis(3));
            }

            loop {
                match rx.try_recv() {
                    Ok(_) => {
                        bar.finish_and_clear();
                        break;
                    }
                    Err(_) => {
                        bar.inc(1);
                        thread::sleep(time::Duration::from_micros(1));
                    }
                }
            }
        });
        jh.join().unwrap();

        return Ok(());

        fn bar() -> ProgressBar {
            let bar = ProgressBar::new_spinner();
            bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{prefix:.green}{spinner:.green} {msg:.green}")
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );
            bar.set_prefix(format!("{:>4}", " "));
            bar.set_message("searching...".to_string());
            bar
        }
    }

    pub fn save(&self) -> Result<()> {
        let db = util::open_db()?;
        db.insert(self.phrase.as_str(), self.raw.clone().unwrap())?;

        Ok(())
    }

    pub fn display(&self) {
        match self.engine {
            Engines::Bing => todo!(),
            Engines::Youdao => {
                let v = self.raw.as_ref().unwrap();
                let blob: youdao::YoudaoRes = serde_json::from_slice(v).unwrap();
                let vb = VocabBody::from(blob);
                println!("{}", vb);
            }
        }
        if let Some(v) = &self.vocabulary {
            println!("{:?}", v);
        }
    }

    fn from_cache(&self) -> Result<Option<Vec<u8>>> {
        let raw = util::open_db()?.get(&self.phrase).ok().unwrap_or(None);
        Ok(raw.map(|ivec| ivec.deref().to_vec()))
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (k, v) in self.0.iter().enumerate() {
            writeln!(
                f,
                "{space:>4}{index:>2}. {value}",
                space = ' ',
                index = util::coloring((k + 1).to_string().as_str(), util::ColorfulRole::Index),
                value = util::coloring(v.as_str(), util::ColorfulRole::Content)
            )?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for History {
    fn from(s: Vec<String>) -> Self {
        History(s)
    }
}

pub fn show_history(length: usize) -> Vec<String> {
    let db = util::open_db();
    if db.is_err() {
        return Vec::new();
    }

    let db = db.unwrap();
    let mut ivecs: Vec<sled::IVec> = vec![];

    for (i, key) in db.iter().keys().enumerate() {
        if i == length {
            break;
        }
        if let Ok(v) = key {
            ivecs.push(v)
        }
    }

    let res: Vec<String> = ivecs
        .into_iter()
        .filter_map(|v| String::from_utf8(v.as_ref().to_vec()).ok())
        .collect();

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_target() {
        let target = QueryTarget::new("hello".to_string());
        assert_eq!(target.phrase, "hello");
        assert!(target.vocabulary.is_none());
        assert!(target.raw.is_none());
        assert_eq!(target.engine, Engines::Youdao);
    }

    #[test]
    fn test_get_cache() {
        let target = QueryTarget::new("x".to_string());
        let c = target.from_cache().unwrap();
        assert!(c.is_some());
    }

    #[test]
    fn test_history() {
        let h = show_history(5);
        assert_eq!(h.len(), 5);
    }
}
