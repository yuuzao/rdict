pub mod youdao;

use std::{fmt, io::Read};

use crate::meta::DictMsg;
use crate::result::Result;
use crate::util::{coloring, ColorfulRole as Role};

#[derive(Debug)]
pub struct VocabBody {
    phrase: String,
    phonetic: Option<Phonetic>,
    explains: Option<Vec<Explain>>,
    examples: Option<Vec<Example>>,
    typo: Option<Vec<Typo>>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Variety {
    US,
    UK,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct PhoneticUri {
    uk: Option<String>,
    us: Option<String>,
}

#[derive(Debug, Clone)]
struct Phonetic {
    us: Option<String>,
    uk: Option<String>,
}

#[derive(Debug, Clone)]
struct Typo {
    pub guessing: Option<String>,
    pub meaning: Option<String>,
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
}

impl fmt::Display for VocabBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "{:>4}{}", " ", coloring(DictMsg::Shrug, Role::Content))?;
            writeln!(
                f,
                "{:>4}{}",
                " ",
                coloring(DictMsg::NotFound, Role::Content)
            )?;
            return Ok(());
        }

        let space = " ";
        let title = |title: &str| coloring(title, Role::Title);
        let index = |index: &str| coloring(index, Role::Index);
        let symbol = |p: &str| coloring(p, Role::Dot);
        let dot = || coloring(". ", Role::Dot);
        let content = |c: &str| coloring(c, Role::Content);
        let emphasis = |word: &str| coloring(word, Role::Emphasis);

        // 音标
        if let Some(p) = self.phonetic.clone() {
            writeln!(f, "{s}{t}", s = space.repeat(4), t = title("音标"))?;
            write!(f, "{s}", s = space.repeat(4))?;
            let mut wp = |us_uk: &str, ph: &str| {
                write!(
                    f,
                    "{s}{zh}{lb}{phonetic}{rb}",
                    s = space.repeat(4),
                    zh = index(ph),
                    lb = symbol("["),
                    phonetic = content(us_uk),
                    rb = symbol("]"),
                )
                .unwrap();
            };

            if let Some(uk) = p.uk {
                wp(uk.as_str(), "英")
            }
            if let Some(us) = p.us {
                wp(us.as_str(), "美")
            }
            writeln!(f)?;
            writeln!(f)?;
        }

        // 释义
        if let Some(exp) = &self.explains {
            writeln!(f, "{s}{t}", s = space.repeat(4), t = title("释义"))?;
            for e in exp.iter() {
                if let Some(i) = e.content.clone().unwrap().split_once('.') {
                    writeln!(
                        f,
                        "{s}{part}{dot}{zh}",
                        s = space.repeat(8),
                        part = index(i.0.trim()),
                        dot = dot(),
                        zh = coloring(i.1.trim(), Role::Content),
                    )?;
                } else {
                    writeln!(
                        f,
                        "{s}{dot}{zh}",
                        s = space.repeat(7),
                        dot = index(">> "),
                        zh = content(e.content.clone().unwrap().trim())
                    )?;
                }
            }
            writeln!(f)?;
        }

        // 例句
        if let Some(exa) = &self.examples {
            writeln!(f, "{s}{t}", s = space.repeat(4), t = title("例句"))?;
            for (i, e) in exa.clone().iter().enumerate() {
                write!(
                    f,
                    // align issue if index large than 10, but this will never happen
                    "{s}{index}{dot}",
                    s = space.repeat(8),
                    index = index((i + 1).to_string().as_str()),
                    dot = dot(),
                )?;

                let phrase = self.phrase.clone().to_lowercase();
                let mut sentence_eng = String::new();
                for v in e.sentence_eng.split(' ') {
                    let x = v.to_lowercase();
                    if x.starts_with(phrase.as_str()) || x.ends_with(phrase.as_str()) {
                        sentence_eng.push_str(emphasis(v).as_str());
                    } else {
                        sentence_eng.push_str(content(v).as_str());
                    }
                    sentence_eng.push(' ');
                }
                writeln!(f, "{}", sentence_eng)?;
                writeln!(
                    f,
                    "{p:>11}{sentence_cn}",
                    p = " ",
                    sentence_cn = coloring(e.trans.as_str(), Role::Other)
                )?;
            }
        } else if let Some(typo) = &self.typo {
            // typo
            writeln!(
                f,
                "{:>4}{phrase} {msg}",
                ' ',
                phrase = emphasis(&*self.phrase.clone()),
                msg = content("may be a typo, are you looking for:"),
            )?;
            for w in typo {
                if let Some(g) = &w.guessing {
                    writeln!(
                        f,
                        "{s}{t} {word}",
                        s = space.repeat(7),
                        t = coloring(">", Role::Dot),
                        word = emphasis(g.as_str()),
                    )?;
                }
                if let Some(m) = &w.meaning {
                    if let Some(i) = m.split_once('.') {
                        writeln!(
                            f,
                            "{s}{part}{dot}{m}",
                            s = space.repeat(8),
                            part = index(i.0),
                            dot = dot(),
                            m = content(i.1.trim()),
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
pub trait Query {
    fn query_meaning(&self, text: &str) -> Result<Vec<u8>>;
    // fn query_pronounce(&self, text: Option<&str>) -> Result<PhoneticUri>;
}
