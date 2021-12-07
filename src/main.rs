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

fn main() {
    simple_logger::init_with_env().expect("cannot initialize logger");

    let args = Rlox::from_args();
    if let Some(path) = args.path {
        run_file(&path);
    } else {
        repl();
    }

    std::process::exit(0);
}

fn repl() {
    log::debug!("launching repl");

    log::debug!("terminating repl");
}

fn run_file<P>(path: &P)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    log::debug!("running file at {:?}", path);

    log::debug!("finished running file");
}
