use bulut::machine::Machine;
use osmon::{
    parser::{lex, parse},
    Compiler,
};

use clap::Parser;
use std::{fs::File, io::prelude::*, path::PathBuf, process::exit};

/// O'zbeklar uchun yaratilgan dinamik til
#[derive(Parser, Debug)]
#[clap(version, about = "O'zbeklar uchun yaratilgan dinamik til", long_about = None)]
pub struct Options {
    /// Ishga tushurili kerak bo'lgan ushbu fayl
    #[clap(name = "FAYL", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Qadamlar ko'rsatilishi kerakligi
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let mut src = String::new();

    let ops = Options::parse();

    if let Some(path) = ops.file {
        File::open(path).unwrap().read_to_string(&mut src).unwrap();
    } else {
        println!("\x1b[93mDastur turgan fayl joyini ko'rsating!\x1b[0m\nKo'proq ma'lumot uchun \"osmon -h\" yozing...");
        exit(1);
    }

    let lex = lex(&src);
    let parsed = parse(&mut lex.peekable()).unwrap();
    let mut machine = Machine::new();
    let mut compiler = Compiler::new(&mut machine, 0, ops.debug);
    compiler.compile(parsed);
}
