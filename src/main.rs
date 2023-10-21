use std::{
    io::{stdin, stdout},
    path::PathBuf,
};

use anyhow::Ok;
use brainfuck_rs::vm::VM;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[clap(name = "FILE")]
    source_file: PathBuf,

    #[clap(short = 'o', long = "optimize", help = "Optimize code")]
    optimize: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Cli::parse();
    VM::new(
        &opt.source_file,
        Box::new(stdin().lock()),
        Box::new(stdout().lock()),
        opt.optimize,
    ).and_then(|mut vm|vm.run())?;
    Ok(())
}
