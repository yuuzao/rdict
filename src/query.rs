use std::io::BufReader;
use std::{fmt, ops::Deref, sync::mpsc, thread, time};

use indicatif::{ProgressBar, ProgressStyle};
use rodio::{source::Source, Decoder, OutputStream};

use crate::handler::{youdao, AudioType, Engines, VocabBody};
use crate::result::Result;
use crate::util::{self, ColorfulRole as Role, Style};

#[allow(dead_code)]
pub struct QueryTarget {
    pub engine: Engines,
    pub phrase: String,
    pub vocabulary: Option<VocabBody>,
    raw: Option<Vec<u8>>,
    audio_uk: Option<Vec<u8>>,
    audio_us: Option<Vec<u8>>,
}

pub struct History(Vec<String>);

impl QueryTarget {
    pub fn new(phrase: String, engine: Engines) -> Self {
        QueryTarget {
            phrase,
            engine,
            vocabulary: None,
            raw: None,
            audio_uk: None,
            audio_us: None,
        }
    }

    pub fn query_meaning(&mut self) -> &Self {
        // unwrap errors here
        self.raw = self
            .from_cache(&self.phrase)
            .unwrap()
            .or_else(|| self.engine.request_meaning(&self.phrase).ok());

        self
    }

    pub fn query_with_pb(&mut self) -> &Self {
        let (tx, rx) = mpsc::channel();

        self.query_meaning();
        if self.raw.is_some() {
            tx.send(1).unwrap();
        }

        // let's spin
        let jh = thread::spawn(move || {
            println!();
            let bar = bar();

            // just spin for 100 times before continue.
            for _ in 0..100 {
                bar.inc(1);
                thread::sleep(time::Duration::from_millis(3));
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
        });
        jh.join().unwrap();

        return self;

        fn bar() -> ProgressBar {
            let bar = ProgressBar::new_spinner();
            bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{prefix:.green}{spinner:.green} {msg:.green}")
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );
            bar.set_prefix(format!("{:>4}", " "));
            bar.set_message("searching...".to_string());
            bar
        }
    }

    pub fn play_audio(&mut self, t: AudioType) -> Result<()> {
        let data = self.query_audio(t)?;

        let cs = std::io::Cursor::new(data.unwrap());

        let source = Decoder::new(cs.to_owned())?;

        let mut buf = BufReader::new(cs);
        let dura = mp3_duration::from_read(&mut buf)?;

        let (_stream, stream_handler) = OutputStream::try_default()?;
        stream_handler.play_raw(source.convert_samples())?;

        thread::sleep(dura);

        Ok(())
    }

    fn query_audio(&mut self, t: AudioType) -> Result<Option<Vec<u8>>> {
        // TODO: not unwrap errors here

        let v: &str = t.to_owned().into();
        let key = format!("{}_{}", &self.phrase, v);

        let source = self
            .from_cache(key.as_str())
            .unwrap()
            .or_else(|| self.engine.request_audio(&self.phrase, t).ok());

        Ok(source)
    }

    pub fn save(&self) -> Result<&Self> {
        let db = util::open_db()?;
        //TODO: rename audio files by appending suffix like x_us.mp3
        db.insert(self.phrase.as_str(), self.raw.clone().unwrap())?;

        Ok(self)
    }

    fn from_cache(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let raw = util::open_db()?.get(key).ok().unwrap_or(None);
        Ok(raw.map(|ivec| ivec.deref().to_vec()))
    }
}

impl fmt::Display for QueryTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.engine {
            Engines::Bing => todo!(),
            Engines::Youdao => {
                let v = self.raw.as_ref().unwrap();
                let data: youdao::YoudaoRes = serde_json::from_slice(v).unwrap();
                let vb = VocabBody::from(data);
                write!(f, "{}", vb)?;
            }
        }
        if let Some(v) = &self.vocabulary {
            println!("{:?}", v);
        }

        Ok(())
    }
}

impl History {
    pub fn getn(length: usize) -> Self {
        let mut res: Vec<String> = Vec::new();
        if let Ok(db) = util::open_db() {
            let mut ivecs: Vec<sled::IVec> = vec![];

            for (i, key) in db.iter().keys().enumerate() {
                if i == length {
                    break;
                }
                if let Ok(v) = key {
                    ivecs.push(v)
                }
            }

            res = ivecs
                .into_iter()
                .filter_map(|v| String::from_utf8(v.as_ref().to_vec()).ok())
                .collect();
        }
        res.into()
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (k, v) in self.0.iter().enumerate() {
            writeln!(
                f,
                "{s}{index}{dot}{value}",
                s = " ".repeat(4),
                index = (k + 1).align_right(2).coloring(Role::Index),
                dot = ".".align_left(2).coloring(Role::Dot),
                value = v.coloring(Role::Content),
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

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_get_cache() {
        let target = QueryTarget::new("x".to_string(), Engines::from("youdao".to_string()));
        let c = target.from_cache(&target.phrase).unwrap();
        assert!(c.is_some());
    }

    #[test]
    #[serial]
    fn test_history() {
        let h = History::getn(0);
        assert_eq!(h.0.len(), 0);
    }

    #[test]
    #[serial]
    fn test_audio() {
        let mut target = QueryTarget::new("hello".to_string(), Engines::from("youdao".to_string()));

        let audio = target.query_audio(AudioType::US).unwrap();
        assert!(audio.is_some());

        let audio = std::io::Cursor::new(audio.unwrap());

        let mut deco = Decoder::new(audio).unwrap();
        assert!(deco.any(|x| x != 0));
    }
}
