use crate::handler::{youdao, VocabBody};
use std::io;

#[derive(Debug)]
pub enum Engines {
    Youdao,
}

pub struct QueryTarget {
    pub engine: Engines,
    pub phrase: String,
}

impl QueryTarget {
    pub fn query(&self) -> io::Result<VocabBody> {
        let ship = match self.engine {
            Engines::Youdao => youdao::Youdao::new(self.phrase.as_str()),
        };
        let res = ship.query_meaning().unwrap();
        Ok(res)
    }
}
