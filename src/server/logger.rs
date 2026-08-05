use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

pub fn init_logging<S: AsRef<str>>(level: S) -> (WorkerGuard, WorkerGuard) {
    // Wyciągamy &str z generyka S
    let level_ref = level.as_ref();

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level_ref));

    // Konsola
    let (non_blocking_stdout, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_stdout)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .with_ansi(true)
        .with_target(false);

    // Plik
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking_file)
        .with_ansi(false)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .with_target(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    (stdout_guard, file_guard)
}
