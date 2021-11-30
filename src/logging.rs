extern crate chrono;
extern crate env_logger;

use chrono::Local;
use std::io::Write;
use env_logger::Builder;

pub fn setup(level: log::LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S.%3f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, level)
        .init();
}
