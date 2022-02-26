use crate::meta::Meta;
use crate::query::{Engines, QueryTarget};
use clap::{ColorChoice, ErrorKind as CError, Parser};
use colored::Colorize;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about  = None)]
pub struct Args {
    /// What do you want to query?
    #[clap(short, long, multiple_values = true)]
    phrase: Vec<String>,

    /// Where do you want to query from?
    #[clap(
        short,
        long,
        default_value = "youdao",
        possible_values= ["youdao","bing"]
    )]
    dict: String,
}

pub fn parse_args() -> Result<QueryTarget, ArgError> {
    use std::env;
    let mut input: Vec<_> = env::args_os().map(|v| v.into_string().unwrap()).collect();

    if input.len() == 1 {
        Meta::show_logo();
        std::process::exit(0);
    } else {
        let mut ph = input.get(1).unwrap();
        // make sure the first string is not an argument, but do not validate it
        if !ph.starts_with('-') {
            input.insert(1, "-p".to_string())
        }
    }

    let args = Args::parse_from(input);

    match args.phrase.is_empty() {
        true => {
            Meta::show_usage();
            Err(wrap_error(ArgError::EmptyValue))
        }
        false => {
            let mut target = QueryTarget::new(args.phrase.join(" "));
            target.engine = match args.dict.as_str() {
                "bing" => Engines::Bing,
                _ => Engines::Youdao,
            };

            Ok(target)
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
