use std::{env, fmt};

use clap::{ArgGroup, ColorChoice, CommandFactory, ErrorKind as CError, Parser};
use colored::Colorize;

use crate::query::{Engines, QueryTarget};
use crate::result::Result;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about  = None)]
#[clap(group(ArgGroup::new("query").args(&["phrase", "dict"]).multiple(true).requires("phrase")))]
#[clap(group(ArgGroup::new("function").args(&["list"]).conflicts_with_all(&["query"])))]
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

    /// list query history
    #[clap(short, long, default_missing_value = "5")]
    list: Option<usize>,
}

pub enum CliAction {
    Query(String, String), //phrases and engine
    ListHistory(usize),
    Other,
}

pub fn parse_args() -> Result<CliAction> {
    let input = handle_input();
    let args = Args::parse_from(input);

    if !args.phrase.is_empty() {
        return Ok(CliAction::Query(args.phrase.join(" "), args.dict));
    } else if let Some(list) = args.list {
        return Ok(CliAction::ListHistory(list));
    } else {
        return Ok(CliAction::Other);
    }

    fn handle_input() -> Vec<String> {
        let mut input: Vec<_> = env::args_os().map(|v| v.into_string().unwrap()).collect();

        assert_ne!(input.len(), 0);

        if input.len() == 1 {
            return Vec::new();
        }

        let mut phrase = input.get(1).unwrap();
        if !phrase.starts_with('-') {
            input.insert(1, "-p".to_string());
        }
        input
    }
}
