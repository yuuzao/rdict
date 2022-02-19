use super::*;
use anyhow;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
struct Trs {
    tr: Vec<TrsTr>,
}
#[derive(Deserialize, Clone, Debug)]
struct TrsTr {
    l: TrsTrL,
}
#[derive(Deserialize, Clone, Debug)]
struct TrsTrL {
    i: Vec<String>,
}
#[derive(Deserialize, Clone, Debug)]
struct Data {
    ukphone: Option<String>,
    usphone: Option<String>,
    // #[serde(flatten)]
    trs: Vec<Trs>,
}
#[derive(Deserialize, Debug)]
struct YoudaoEc {
    word: Vec<Data>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct SentencePair {
    sentence: String,
    sentence_translation: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct YoudaoExample {
    sentence_pair: Vec<SentencePair>,
}

#[derive(Deserialize, Debug)]
struct YoudaoResMeta {
    // "input": "xyz",
    // "guessLanguage": "eng",
    // "isHasSimpleDict": "1",
    // "le": "en",
    // "lang": "eng",
    // "dicts": [
    // "meta",
    // "ec",
    // "blng_sents_part"
    // ]
    lang: String,
    input: String,
    dicts: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct YoudaoRes {
    meta: YoudaoResMeta,
    ec: YoudaoEc,
    blng_sents_part: YoudaoExample,
}
// --------------------

#[derive(Clone, Debug)]
pub struct Youdao {
    phrase: String,
}

impl Youdao {
    pub fn new(phrase: &str) -> Self {
        Youdao {
            phrase: phrase.to_lowercase(),
        }
    }

    pub fn query_meaning(&self) -> Result<VocabBody, QueryError> {
        let phrase_url = Url::parse_with_params(
            "http://dict.youdao.com/jsonapi?",
            &[("q", self.phrase.clone())],
        )?;

        let res: YoudaoRes = reqwest::blocking::get(phrase_url)?.json()?;

        Ok(VocabBody::from(res))
    }

    /// The youdao API accepts any non-empty phrase, such as r11ust, interesting.
    pub fn query_pronounce(&self) -> Result<PhoneticUri, QueryError> {
        let mut url = Url::parse_with_params(
            "https://dict.youdao.com/dictvoice?",
            &[("audio", self.phrase.clone())],
        )
        .unwrap();
        let mut uk_url = url.clone();
        uk_url.set_query(Some("type=1"));
        url.set_query(Some("type=2"));

        Ok(PhoneticUri {
            uk: uk_url.to_string(),
            us: url.to_string(),
        })
    }
}

impl From<YoudaoRes> for VocabBody {
    fn from(ydr: YoudaoRes) -> VocabBody {
        let mut explains = vec![];
        let mut examples = vec![];

        for e in ydr.ec.word[0].trs.clone() {
            explains.push(Explain {
                content: Some(e.tr[0].l.i[0].clone()),
            });
        }
        for e in ydr.blng_sents_part.sentence_pair {
            examples.push(Example {
                sentence_eng: e.sentence,
                trans: e.sentence_translation,
            });
        }

        VocabBody {
            phrase: Some(ydr.meta.input.clone()),
            phonetic: Phonetic {
                uk: ydr.ec.word[0].ukphone.clone(),
                us: ydr.ec.word[0].usphone.clone(),
            },
            explains,
            examples,
        }
    }
}

impl Query for Youdao {
    fn query_meaning(&self, text: Option<&str>) -> Result<VocabBody, QueryError> {
        match text {
            None => Err(QueryError::InputError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "empty input",
            ))),
            Some(t) => {
                let ydr = Youdao::new(t).query_meaning()?;
                Ok(VocabBody::from(ydr))
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
        yd.query_pronounce()
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
}
