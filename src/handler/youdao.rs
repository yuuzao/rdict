use super::*;
use serde::Deserialize;
use std::fmt::Debug;

// json root
#[derive(Deserialize, Debug, Clone)]
pub struct YoudaoRes {
    meta: Meta,
    #[serde(alias = "ce")]
    ec: Option<EC>, // ec: english-chinese, ce: chinese-english
    typos: Option<Typo>,
    blng_sents_part: Option<BlngSentsPart>, // examples
}

// root > meta
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Meta {
    lang: String,
    guess_language: String,
    input: String,
    dicts: Vec<String>,
}

// root > ec
#[derive(Deserialize, Debug, Clone)]
struct EC {
    word: Vec<Word>,
}

// root > ec > word[]
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
struct Word {
    usphone: Option<String>,
    ukphone: Option<String>,
    trs: Option<Vec<Trs>>,
    return_phrase: Option<Phrase>,
}

// root > ec > word > trs[]
#[derive(Deserialize, Clone, Debug)]
struct Trs {
    tr: Vec<TrsTr>,
}

// root > ec > word > trs[] > tr[]
#[derive(Deserialize, Clone, Debug)]
struct TrsTr {
    l: TrsTrL,
}

// root > ec > word > trs[] > tr[] > l
#[derive(Deserialize, Clone, Debug)]
struct TrsTrL {
    pos: Option<String>, // chinese-english only
    i: Vec<serde_json::Value>,
}
impl TrsTrL {
    fn extract(&self, lang: &str) -> String {
        if lang == "zh" {
            let mut s = String::new();
            if let Some(x) = &self.pos {
                s.push_str(x.as_str())
            }

            for value in &self.i {
                if let Some(v) = value.as_str() {
                    if !v.is_empty() {
                        s.push_str(v)
                    }
                } else if let Some(v) = value.as_object() {
                    s.push_str(v["#text"].as_str().unwrap());
                }
            }
            s
        } else {
            self.i[0].as_str().unwrap().to_string()
        }
    }
}

// root > ec > word > retrun-phrase
#[derive(Deserialize, Clone, Debug)]
struct Phrase {
    l: PhraseL,
}

// root > ec > word > retrun-phrase > l
#[derive(Deserialize, Clone, Debug)]
struct PhraseL {
    i: String,
}

// root > sentence_eng
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
struct SentencePair {
    sentence: String,
    sentence_translation: String,
}

// root > blng_sents_part
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
struct BlngSentsPart {
    sentence_pair: Vec<SentencePair>,
}

// root > typos
#[derive(Deserialize, Debug, Clone)]
struct Typo {
    typo: Vec<TypoContent>,
}

// root > typos > typo[]
#[derive(Deserialize, Debug, Clone)]
struct TypoContent {
    word: Option<String>,
    trans: Option<String>,
}

impl From<YoudaoRes> for VocabBody {
    fn from(ydr: YoudaoRes) -> VocabBody {
        // Responsed phrase may contain mixed cases. eg: "british" -> "British"
        let mut vb = VocabBody::new(ydr.meta.input);

        if let Some(t) = ydr.typos {
            let mut z = Vec::new();
            for v in t.typo.iter() {
                z.push(super::Typo {
                    guessing: v.word.clone(),
                    meaning: v.trans.clone(),
                })
            }
            vb.typo = Some(z);
        }
        if let Some(ec) = ydr.ec {
            let mut explains = vec![];
            let mut examples = vec![];

            if let Some(p) = &ec.word[0].return_phrase {
                vb.phrase = p.l.i.clone();
            }

            if let Some(trs) = &ec.word[0].trs {
                for e in trs {
                    explains.push(Explain {
                        content: Some(e.tr[0].l.extract(&ydr.meta.guess_language)),
                    });
                }
            }

            let valid_phonetic = |mut p: Option<String>| {
                if let Some(x) = &p {
                    if x.is_empty() {
                        p = None
                    }
                };
                p
            };
            let ukp = ec.word[0].ukphone.clone();
            let usp = ec.word[0].usphone.clone();
            if !(valid_phonetic(ukp).is_none() && valid_phonetic(usp).is_none()) {
                vb.phonetic = Some(Phonetic {
                    uk: ec.word[0].ukphone.clone(),
                    us: ec.word[0].usphone.clone(),
                })
            }

            if let Some(part) = ydr.blng_sents_part {
                for e in part.sentence_pair {
                    examples.push(Example {
                        sentence_eng: e.sentence,
                        trans: e.sentence_translation,
                    });
                }
            }

            use std::ops::Not;
            explains
                .is_empty()
                .not()
                .then(|| vb.explains = Some(explains));
            examples
                .is_empty()
                .not()
                .then(|| vb.examples = Some(examples));
        }

        vb
    }
}
