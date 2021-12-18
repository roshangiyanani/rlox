use std::path::Path;

use anyhow::{Context, Error};
use structopt::StructOpt;

use crate::compiler::scanner::Scanner;

mod bytecode;
mod compiler;
mod dissembler;
mod value;
mod vm;

#[derive(Debug, StructOpt)]
/// A rust implemenation of a lox interpreter/compiler/vm.
/// If no file path is given, drops into a REPL.
struct Rlox {
    /// a file to run
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    simple_logger::init_with_env().expect("cannot initialize logger");

    let args = Rlox::from_args();
    let result = match args.path {
        Some(path) => run_file(&path),
        None => repl(),
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            log::error!("{:?}", error);
            std::process::exit(1)
        }
    }
}

fn repl() -> Result<(), Error> {
    log::debug!("launching repl");

    let mut line = String::new();
    let stdin = std::io::stdin();
    loop {
        let read_size = stdin
            .read_line(&mut line)
            .with_context(|| "unable to read input")?;
        if read_size == 0 {
            log::debug!("received EOF");
            break;
        }

        // strip newline
        let trimmed_line = line.trim_end();
        log::trace!("input: \"{}\"", trimmed_line);

        // todo: interpret
        line.clear();
    }

    log::debug!("terminating repl");
    Ok(())
}

fn run_file<P>(path: &P) -> Result<(), Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("unable to read lox file at {:?}", path))?;
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("read file at {:?}:\n{}", path, source);
    } else {
        log::info!("read file at {:?}", path)
    }

    let mut scanner = Scanner::new(&source);
    for (loc, parsed) in &mut scanner {
        let token = parsed.with_context(|| format!("scanner error at {}", loc))?;
        log::trace!("{:}: {:?}", loc, token);
    }

    log::debug!("finished running file");
    Ok(())
}
