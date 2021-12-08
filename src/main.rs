use std::path::Path;

use structopt::StructOpt;

mod bytecode;
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

fn main() -> ! {
    simple_logger::init_with_env().expect("cannot initialize logger");

    let args = Rlox::from_args();
    let result = match args.path {
        Some(path) => run_file(&path),
        None => repl(),
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            log::error!("{}", error);
            std::process::exit(1)
        }
    }
}

fn repl() -> Result<(), std::io::Error> {
    log::debug!("launching repl");

    let mut line = String::new();
    let stdin = std::io::stdin();
    loop {
        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            log::debug!("received EOF");
            break;
        }

        // strip newline
        let line = line.trim_end();
        log::debug!("input: \"{}\"", &line[..line.len() - 1]);

        // todo: interpret
    }

    log::debug!("terminating repl");
    Ok(())
}

fn run_file<P>(path: &P) -> Result<(), std::io::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    log::debug!("running file at {:?}", path);
    let file = std::fs::read_to_string(path)?;
    log::debug!("read file:\n{}", file);

    log::debug!("finished running file");
    Ok(())
}
