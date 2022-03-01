use crate::handler::{youdao, Query, QueryError, VocabBody};
use crate::util;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{io::Result, ops::Deref, sync::mpsc, thread, time};

#[derive(Debug)]
pub enum Engines {
    Youdao,
    Bing,
}

impl From<String> for Engines {
    fn from(eng: String) -> Self {
        match eng.as_str() {
            "bing" => Engines::Bing,
            _ => Engines::Youdao,
        }
    }
}

pub struct QueryTarget {
    pub engine: Engines,
    pub phrase: String,
    pub vocabulary: Option<VocabBody>,
    raw: Option<Vec<u8>>,
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

    pub fn query(&mut self) -> std::result::Result<(), QueryError> {
        let t = match self.engine {
            Engines::Bing => todo!(),
            _ => youdao::Youdao::new(self.phrase.as_str()),
        };
        self.raw = if let Some(raw) = self.query_local_db()? {
            Some(raw)
        } else {
            let res = t.query_meaning(&self.phrase).unwrap();
            Some(res)
        };
        Ok(())
    }

    pub fn query_with_pb(&mut self) -> std::result::Result<(), QueryError> {
        let (tx, rx) = mpsc::channel();
        if self.query().is_ok() {
            tx.send(1).unwrap();
        }
        thread::spawn(move || {
            println!();
            let bar = ProgressBar::new_spinner();
            bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{prefix:.green}{spinner:.green} {msg:.green}")
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );
            bar.set_prefix(format!("{:>4}", " "));
            bar.set_message("searching...".to_string());
            for _ in 0..100 {
                bar.inc(1);
                thread::sleep(time::Duration::from_millis(2));
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
        })
        .join()
        .unwrap();

        Ok(())
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

    fn query_local_db(&mut self) -> Result<Option<Vec<u8>>> {
        let db = util::open_db()?;
        if let Some(v) = db.get(&self.phrase)? {
            let r: Vec<u8> = v.deref().to_vec();
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }
}

pub struct History(Vec<String>);

use std::fmt;
impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (k, v) in self.0.iter().enumerate() {
            writeln!(
                f,
                "{space:>4}{index}. {value}",
                space = ' ',
                index = k.to_string().truecolor(0, 175, 175),
                value = v.truecolor(30, 250, 110)
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
    let db: sled::Db = util::open_db().unwrap();

    let mut ivecs: Vec<sled::IVec> = vec![];
    for (i, key) in db.iter().keys().enumerate() {
        if i > length {
            break;
        }
        ivecs.push(key.unwrap())
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
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_db() {
        let test_str = "upupdowndownleftleftrightrightBABA";
        let mut target = QueryTarget::new(test_str.to_string());
        let value = vec![12, 34];

        target.raw = Some(value.clone());
        target.save().unwrap();

        let db = util::open_db().unwrap();

        assert!(show_history(usize::MAX).contains(&test_str.to_string()));

        let r = db.remove(test_str).unwrap().unwrap().as_ref().to_vec();
        assert_eq!(value, r);

        let r = db.remove(test_str).unwrap();
        assert_eq!(None, r);
    }
}
