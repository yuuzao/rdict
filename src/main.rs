#[allow(unused, dead_code)]
mod args;
mod handler;
mod query;

use args::handle_args;
fn main() {
    let ags = handle_args().unwrap();
    let output = ags.query().unwrap();
    println!("{}", output);
}
