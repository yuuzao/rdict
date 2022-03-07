mod args;
mod handler;
mod meta;
mod query;
mod result;
mod util;

use args::{parse_args, CliAction};
use query::{History, QueryTarget};

fn main() {
    match parse_args().unwrap() {
        CliAction::Query(info) => {
            let mut target = QueryTarget::new(info.phrase, info.engine);
            target.query_with_pb().save().unwrap();
            println!("{}", target);

            if let Some(v) = info.voice {
                target.play_audio(v).unwrap();
            }
        }
        CliAction::ListHistory(s) => {
            let history = History::getn(s);
            println!("{}", history);
        }
        CliAction::Other => meta::show_logo(),
    }
}
