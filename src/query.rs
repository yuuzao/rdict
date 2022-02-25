use crate::handler::{youdao, VocabBody};

#[derive(Debug)]
pub enum Engines {
    Youdao,
}

pub struct QueryTarget {
    pub engine: Engines,
    pub phrase: String,
}

impl QueryTarget {
    pub fn query(&self) -> Option<VocabBody> {
        let ship = match self.engine {
            Engines::Youdao => youdao::Youdao::new(self.phrase.as_str()),
        };
        let res = ship.query_meaning();
        match res {
            Err(_) => {
                // TODO: more elegant error handle.
                None
            }
            Ok(vb) => Some(vb),
        }
    }
}
