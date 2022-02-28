use crate::handler::{youdao, Query, QueryError, VocabBody};
use crate::util;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Result;
use std::ops::Deref;
use std::sync::mpsc;
use std::thread;
use std::time;

#[derive(Debug)]
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
            println!("local");
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
            bar.set_message(format!("searching..."));
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
            return Ok(Some(r));
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_save() {
        let test_str = "upupdowndownleftleftrightrightBABA";
        let mut target = QueryTarget::new(test_str.to_string());
        let v = vec![12, 34];
        target.raw = Some(v.clone());
        target.save().unwrap();

        let db = util::open_db().unwrap();
        assert!(db.contains_key(test_str).unwrap());
        let r = db.remove(test_str).unwrap().unwrap().as_ref().to_vec();
        assert_eq!(v, r);
        let r = db.remove(test_str).unwrap();
        assert_eq!(None, r);
    }
}
