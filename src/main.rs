#[allow(unused, dead_code)]
mod args;
mod handler;
mod meta;
mod query;
mod util;

use args::{parse_args, CliAction};
use query::{Engines, History, QueryTarget};

fn main() {
    match parse_args().unwrap() {
        CliAction::Query(phrases, engine) => {
            let mut target = QueryTarget::new(phrases);
            target.engine = Engines::from(engine);

            target.query_with_pb().unwrap();
            target.display();
            target.save().unwrap();
        }
        CliAction::ListHistory(s) => {
            let history: History = query::show_history(s).into();
            println!("{}", history);
        }
        CliAction::Other => meta::show_logo(),
    }
}
