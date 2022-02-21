use crate::meta::Meta;
use crate::query::{Engines, QueryTarget};
use clap::{ColorChoice, ErrorKind as CError, Parser};
use colored::Colorize;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about  = None)]
pub struct Args {
    #[clap(short, long, multiple_values = true)]
    phrase: Vec<String>,

    /// name
    #[clap(short, long, default_value = "youdao")]
    dict: String,
}

pub fn handle_args() -> Result<QueryTarget, ArgError> {
    use std::env;
    let mut input: Vec<_> = env::args_os().map(|v| v.into_string().unwrap()).collect();
    if input.len() == 1 {
        // show_usage();
        Meta::show_logo();
        std::process::exit(0);
    } else {
        let mut ph = input.get(1).unwrap();
        if !ph.starts_with('-') {
            input.insert(1, "-p".to_string())
        }
    }

    let args = Args::try_parse_from(input);

    match args {
        Err(e) => Err(wrap_error(ArgError::ClapError(e.kind()))),
        Ok(a) => {
            if a.phrase.is_empty() {
                Meta::show_usage();
                return Err(wrap_error(ArgError::EmptyValue));
            }

            Ok(QueryTarget {
                engine: match a.dict.as_str() {
                    "youdao" => Engines::Youdao,
                    _ => Engines::Youdao,
                },
                phrase: a.phrase.join(" "),
            })
        }
    }
}

fn wrap_error(e: ArgError) -> ArgError {
    println!("{} ", e);
    e
}

#[derive(Debug)]
pub enum ArgError {
    ClapError(CError),
    EmptyValue,
}
impl From<CError> for ArgError {
    fn from(err: CError) -> ArgError {
        ArgError::ClapError(err)
    }
}

impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClapError(x) => println!("{}", x.to_string().red()),
            Self::EmptyValue => println!("{}", "You have to input something!".red()),
        }
        Ok(())
    }
}
