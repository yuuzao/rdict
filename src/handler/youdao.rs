use super::*;
use serde::Deserialize;
use std::fmt::Debug;
use ureq;

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

            for t in &self.i {
                if let Some(v) = t.as_str() {
                    if !v.is_empty() {
                        s.push_str(v)
                    }
                } else if let Some(v) = t.as_object() {
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

// sentence_eng ----------------------------
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

// --------------------

const PHRASE_API: &str = "http://dict.youdao.com/jsonapi";

#[derive(Clone, Debug, Default)]
pub struct Youdao {
    phrase: String,
}

impl Youdao {
    pub fn new(phrase: &str) -> Self {
        Youdao {
            phrase: phrase.to_lowercase(),
        }
    }
}

impl From<YoudaoRes> for VocabBody {
    fn from(ydr: YoudaoRes) -> VocabBody {
        // the looked up phrase may be converted into mixed case by the API.
        let mut vb = VocabBody::new(ydr.meta.input);

        if let Some(t) = ydr.typos {
            let typos = t.typo;
            let mut z = vec![];
            for v in typos.iter() {
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

impl Query for Youdao {
    fn query_meaning(&self, phrase: &str) -> Result<Vec<u8>, QueryError> {
        use std::io::BufReader;
        match ureq::get(PHRASE_API).query("q", &self.phrase).call() {
            Err(e) => Err(QueryError::RequestError(Box::new(e))),
            Ok(v) => {
                let mut res = vec![];
                v.into_reader().read_to_end(&mut res);
                Ok(res)
            }
        }
    }
    fn query_pronounce(&self, text: Option<&str>) -> Result<PhoneticUri, QueryError> {
        let yd = match text {
            None => {
                return Err(QueryError::InputError(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "empty input",
                )))
            }
            Some(t) => Youdao::new(t),
        };
        unimplemented!();
        // yd.query_pronounce()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_query_meaning() {
    //     let yd = Youdao::new("rust");
    //     assert_eq!(
    //         r#"VocabBody { phrase: Some("rust"), engine: Youdao, phonetic: Phonetic { us: Some("rʌst"), uk: Some("rʌst") }, explains: [Explain { content: Some("n. 锈，铁锈；（植物的）锈病，锈菌；铁锈色，赭色") }, Explain { content: Some("v. （使）生锈；成铁锈色；（因疏忽或不用而）衰退，变迟钝；（因长时间不用或有害使用而）损害，腐蚀") }, Explain { content: Some("【名】 （Rust）（英）拉斯特，（德、捷、瑞典）鲁斯特，（法）吕斯特（人名）") }], examples: [Example { sentence_eng: "<b>Rust</b> had eaten into the metal.", trans: "这金属已经锈坏。" }, Example { sentence_eng: "Brass doesn't <b>rust</b>.", trans: "黄铜不生锈。" }, Example { sentence_eng: "We provide a 5-year guarantee against <b>rust</b>.", trans: "我们保证5年不生锈。" }] }"#,
    //         format!("{:?}", yd.query_meaning().unwrap())
    //     )
    // }
    // #[test]
    // fn test_typo() {
    //     unimplemented!();
    //     //asu, asx, uu, qq
    //     let yd = Youdao::new("qq");
    //     let res = yd.query_meaning(Some("qq"), false).unwrap();
    //
    //     println!("{:?}", res);
    // }
}
