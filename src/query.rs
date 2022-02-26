use crate::handler::{youdao, Query, QueryError, VocabBody};
use std::io;

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
        self.raw = if let Some(raw) = self.query_local_db() {
            Some(raw)
        } else {
            let res = t.query_meaning(&self.phrase).unwrap();
            Some(res)
        };
        Ok(())
    }

    pub fn try_save(&self) -> io::Result<()> {
        // TODO: path configuration
        let db: sled::Db = sled::open("history").unwrap();
        if let Some(raw) = &self.raw {
            let p = self.phrase.as_str();
            db.insert(p, &*raw.clone()).unwrap();
        }
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

    fn query_local_db(&mut self) -> Option<Vec<u8>> {
        let db: sled::Db = sled::open("history").unwrap();
        use std::ops::Deref;
        if let Some(v) = db.get(&self.phrase).unwrap() {
            let r: Vec<u8> = v.deref().to_vec();
            Some(r)
        } else {
            None
        }
    }
}
