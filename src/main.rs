#[allow(unused, dead_code)]
mod args;
mod handler;
mod meta;
mod query;
mod util;

use args::parse_args;
fn main() {
    let mut target = parse_args().unwrap();
    if let Ok(()) = target.query() {
        target.display();
        target.try_save().unwrap();
    }
}
