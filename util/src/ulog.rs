use time::{format_description};
use flexi_logger::{DeferredNow, Logger, Record, style};

fn __logger_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    let format = format_description::parse(
        "[hour]:[minute]:[second]",
    ).unwrap();
    let time = _now.format(&format);

    write!(
        w,
        "[{}] <{}> {} {}",
        style(level).paint(level.to_string()),
        record.module_path().unwrap_or("<unnamed>"),
        &time,
        style(level).paint(record.args().to_string())
    )
}

pub fn __init_logger() {
    Logger::try_with_env_or_str("info")
        .unwrap()
        .format(__logger_format)
        .set_palette("b1;3;2;4;6".to_string())
        .start()
        .unwrap();
}