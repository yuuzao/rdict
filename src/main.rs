#[allow(unused, dead_code)]
mod args;
mod handler;
mod meta;
mod progressbar;
mod query;
mod util;

use args::parse_args;
use progressbar::query_with_pb;
fn main() {
    let mut target = parse_args().unwrap();
    if let Ok(_) = query_with_pb(&mut target) {
        target.display();
        target.try_save().unwrap();
    }
}
