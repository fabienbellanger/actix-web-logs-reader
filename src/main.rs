use actix_web_logs_reader::process_stdin;
use clap::Clap;

/// Filter and pretty-print Actix-web log file content.
#[derive(Clap)]
#[clap(version = "0.1", author = "Fabien Bellanger <valentil@gmail.com>")]
struct Cli {
    /// Only show messages at or above the specified level.
    ///
    /// You can specify level names (trace, debug, info, warn, error, fatal) or a positive
    /// numeric value.
    #[clap(short, long, default_value = "trace")]
    level: String,

    /// Colorize output.
    ///
    /// Defaults to try if output stream is a TTY.
    #[clap(long = "color", conflicts_with = "no-color")]
    color: bool,

    /// Force no coloring (e.g. terminal doesn't support it).
    #[clap(long = "no-color", conflicts_with = "color")]
    no_color: bool,

    /// Suppress all but legal JSON log lines. By default non-JSON lines
    /// are passed through.
    #[clap(long)]
    strict: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    } else if cli.color || atty::is(atty::Stream::Stdout) {
        colored::control::set_override(true);
    }

    process_stdin(cli.level, cli.strict);
}
