#[allow(unused, dead_code)]
mod args;
mod handler;
mod meta;
mod query;
mod util;

use args::handle_args;
fn main() {
    let ags = handle_args().unwrap();
    if let Some(output) = ags.query() {
        println!("{}", output);
    }
}
