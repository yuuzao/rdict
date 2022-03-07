pub mod youdao;

use std::{fmt, io, io::Read};

use crate::meta::{self, DictMsg};
use crate::result::Result;
use crate::util::{ColorfulRole as Role, Style};

// http://dict.youdao.com/jsonapi?q=keyword
const YD_PHRASE_API: &str = "http://dict.youdao.com/jsonapi";

// http://dict.youdao.com/dictvoice?audio=keyword&type=1
const YD_AUDIO_API: &str = "http://dict.youdao.com/dictvoice";

#[derive(Debug, PartialEq, Eq)]
pub enum Engines {
    Youdao,
    Bing,
}

#[derive(Debug)]
pub struct VocabBody {
    phrase: String,
    phonetic: Option<Phonetic>,
    explains: Option<Vec<Explain>>,
    examples: Option<Vec<Example>>,
    typo: Option<Vec<Typo>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AudioType {
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

#[allow(unreachable_code)]
impl From<String> for Engines {
    fn from(engine: String) -> Self {
        match engine.as_str() {
            "bing" => {
                // FIXME
                meta::wip();
                std::process::exit(0);
                Engines::Bing
            }

            _ => Engines::Youdao,
        }
    }
}

impl Engines {
    pub fn request_meaning(&self, phrase: &str) -> Result<Vec<u8>> {
        let api: ureq::Request = match self {
            Engines::Bing => todo!(),
            Engines::Youdao => ureq::get(YD_PHRASE_API).query("q", phrase),
        };
        self.req(api)
    }

    pub fn request_audio(&self, phrase: &str, t: AudioType) -> Result<Vec<u8>> {
        let t: &str = t.into();
        let api = ureq::get(YD_AUDIO_API)
            .query("audio", phrase)
            .query("type", t);
        self.req(api)
    }

    fn req(&self, req_body: ureq::Request) -> Result<Vec<u8>> {
        match req_body.call() {
            Err(e) => Err(e.into()),
            Ok(v) => {
                let mut res = vec![];
                v.into_reader().read_to_end(&mut res)?;
                Ok(res)
            }
        }
    }
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
            writeln!(
                f,
                "{}{}",
                ' '.align_right(4),
                DictMsg::Shrug.coloring(Role::Content)
            )?;
            writeln!(
                f,
                "{}{}",
                ' '.align_right(4),
                DictMsg::NotFound.coloring(Role::Content)
            )?;
            return Ok(());
        }

        let space = |n| ' '.align_right(n);
        let title = |title: &str| title.coloring(Role::Title);
        let index = |index: &str| index.coloring(Role::Index);
        let symbol = |p: &str| p.coloring(Role::Dot);
        let dot = || ". ".coloring(Role::Dot);
        let content = |c: &str| c.coloring(Role::Content);
        let emphasis = |word: &str| word.coloring(Role::Emphasis);

        // 音标
        if let Some(p) = self.phonetic.clone() {
            writeln!(f, "{s}{t}", s = space(4), t = title("音标"))?;
            write!(f, "{s}", s = space(4))?;
            let mut wp = |us_uk: &str, ph: &str| {
                write!(
                    f,
                    "{s}{zh}{lb}{phonetic}{rb}",
                    s = space(4),
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
            writeln!(f, "{s}{t}", s = space(4), t = title("释义"))?;
            for e in exp.iter() {
                if let Some(i) = e.content.clone().unwrap().split_once('.') {
                    writeln!(
                        f,
                        "{s}{part}{dot}{zh}",
                        s = space(8),
                        part = index(i.0.trim()),
                        dot = dot(),
                        zh = (i.1.trim().coloring(Role::Content)),
                    )?;
                } else {
                    writeln!(
                        f,
                        "{s}{dot}{zh}",
                        s = space(7),
                        dot = index(">> "),
                        zh = content(e.content.clone().unwrap().trim())
                    )?;
                }
            }
            writeln!(f)?;
        }

        // 例句
        if let Some(exa) = &self.examples {
            writeln!(f, "{s}{t}", s = space(4), t = title("例句"))?;
            for (i, e) in exa.clone().iter().enumerate() {
                write!(
                    f,
                    // align issue if index large than 10, but this will never happen
                    "{s}{index}{dot}",
                    s = space(8),
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
                    sentence_cn = e.trans.coloring(Role::Other),
                )?;
            }
        } else if let Some(typo) = &self.typo {
            // typo
            writeln!(
                f,
                "{s}{phrase} {msg}",
                s = space(4),
                phrase = emphasis(&*self.phrase.clone()),
                msg = content("may be a typo, are you looking for:"),
            )?;
            writeln!(f)?;

            for w in typo {
                if let Some(g) = &w.guessing {
                    writeln!(
                        f,
                        "{s}{t}{word}",
                        s = space(4),
                        t = '>'.align_left(2).coloring(Role::Dot),
                        word = emphasis(g.as_str()),
                    )?;
                }
                if let Some(m) = &w.meaning {
                    if let Some(i) = m.split_once('.') {
                        writeln!(
                            f,
                            "{s}{part}{dot}{m}",
                            s = space(6),
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

impl TryFrom<String> for AudioType {
    type Error = io::Error;
    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "1" | "uk" => Ok(AudioType::UK),
            "2" | "us" => Ok(AudioType::US),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "unexped type")),
        }
    }
}

impl From<AudioType> for &str {
    fn from(s: AudioType) -> Self {
        match s {
            AudioType::US => "2",
            AudioType::UK => "1",
        }
    }
}
