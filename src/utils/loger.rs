#![allow(unused_imports)]
use std::time::SystemTime;
use time::macros::format_description;
use tracing::{debug, error, info, trace, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::time::OffsetTime, util::SubscriberInitExt};

pub fn loger_init(verbose: bool) {
    let file_appender = tracing_appender::rolling::hourly("logs/", "rolling.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    let offset = clia_local_offset::current_local_offset().expect("Can not get local offset!");
    let timer = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let tracer = tracing_subscriber::fmt()
        .with_timer(timer.clone())
        .with_writer(file_writer)
        .with_thread_ids(true)
        .with_file(true);

    if verbose {
        tracer.with_writer(std::io::stdout).init();
    } else {
        tracer.init();
    }

    error!("error");
}
