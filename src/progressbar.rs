use crate::query::QueryTarget;
use indicatif::{ProgressBar, ProgressStyle};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

pub fn query_with_pb(target: &mut QueryTarget) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    if let Ok(_) = target.query() {
        tx.send(1).unwrap();
    };
    thread::spawn(move || {
        println!();
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{prefix:.green}{spinner:.green} {msg:.green}")
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        bar.set_prefix(format!("{:>4}", " "));
        bar.set_message(format!("searching..."));
        for _ in 0..250 {
            bar.inc(1);
            thread::sleep(time::Duration::from_millis(2));
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
    })
    .join()
    .unwrap();

    Ok(())
}
