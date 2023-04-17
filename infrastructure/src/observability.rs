use tracing_appender::non_blocking::WorkerGuard;

pub fn init_tracing() -> WorkerGuard {
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_span_list(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking_writer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("failed to init tracing subscriber");

    guard
}
