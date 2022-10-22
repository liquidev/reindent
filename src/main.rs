use std::{fs, io, path::PathBuf};

use clap::Parser;
use tracing::{debug, error, info, info_span, metadata::LevelFilter};
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Indentation {
    Spaces(usize),
    Tab,
}

impl Indentation {
    fn from_line(line: &str) -> Option<Self> {
        if line.starts_with('\t') {
            Some(Self::Tab)
        } else {
            let spaces = line.chars().take_while(|&c| c == ' ').count();
            if spaces > 0 {
                Some(Self::Spaces(spaces))
            } else {
                None
            }
        }
    }

    fn strip_from(self, mut line: &str) -> Option<&str> {
        match self {
            Self::Tab => line.strip_prefix('\t'),
            Self::Spaces(spaces) => {
                for _ in 0..spaces {
                    if let Some(stripped) = line.strip_prefix(' ') {
                        line = stripped;
                    } else {
                        return None;
                    }
                }
                Some(line)
            }
        }
    }

    fn strip_all_from(self, mut line: &str) -> (&str, usize) {
        let mut levels = 0;
        while let Some(stripped) = self.strip_from(line) {
            line = stripped;
            levels += 1;
        }
        (line, levels)
    }

    fn append_to_string(self, s: &mut String) {
        match self {
            Self::Tab => s.push('\t'),
            Self::Spaces(spaces) => {
                for _ in 0..spaces {
                    s.push(' ');
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputMode {
    Stdout,
    InPlace,
}

fn detect_indentation(file: &str) -> Option<Indentation> {
    let detected = file.lines().find_map(Indentation::from_line);
    debug!(level = ?detected, "detected indentation");
    detected
}

fn reindent_file(
    path: PathBuf,
    from: Option<Indentation>,
    to: Indentation,
    output_mode: OutputMode,
) -> Result<(), io::Error> {
    let file = fs::read_to_string(&path)?;

    let from = from.or_else(|| detect_indentation(&file));
    let reindented = if let Some(from) = from {
        let mut reindented = String::new();
        for line in file.lines() {
            let (without_indentation, levels) = from.strip_all_from(line);
            for _ in 0..levels {
                to.append_to_string(&mut reindented);
            }
            reindented.push_str(without_indentation);
            reindented.push('\n'); // TODO: CRLF?
        }
        info!(?from, ?to, "reindented");
        reindented
    } else {
        info!("file is not indented, leaving as is");
        file
    };

    match output_mode {
        OutputMode::Stdout => print!("{}", reindented),
        OutputMode::InPlace => fs::write(&path, &reindented)?,
    }

    Ok(())
}

/// Quickly reindent files, without altering other aspects of formatting.
#[derive(Parser)]
struct Args {
    /// The files to reindent. If omitted, reads from stdin.
    files: Vec<PathBuf>,

    /// Assume indentation type when reindenting files. Auto-detected if omitted.
    #[clap(short = 'F', long, value_parser = parse_indentation)]
    from: Option<Indentation>,
    /// What indentation the output text should have.
    #[clap(short = 'T', long, value_parser = parse_indentation)]
    to: Indentation,

    /// Whether to reindent the files in place rather than catting them to stdout.
    #[clap(short = 'i', long)]
    in_place: bool,
}

fn parse_indentation(s: &str) -> Result<Indentation, String> {
    if let "tab" | "tabs" = s {
        Ok(Indentation::Tab)
    } else {
        Ok(Indentation::Spaces(
            s.parse::<usize>().map_err(|e| e.to_string())?,
        ))
    }
}

fn main() {
    let args = Args::parse();

    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_writer(io::stderr),
        );
    tracing::subscriber::set_global_default(subscriber)
        .expect("cannot set default tracing subscriber");

    let output_mode = if args.in_place {
        OutputMode::InPlace
    } else {
        OutputMode::Stdout
    };

    if args.files.is_empty() {
        info!("reindenting from stdin");
    } else {
        info!("reindenting from list of files");
        for file in args.files {
            let _span = info_span!("reindent", ?file).entered();
            if let Err(error) = reindent_file(file, args.from, args.to, output_mode) {
                error!(%error, "cannot reindent file")
            }
        }
    }
}
