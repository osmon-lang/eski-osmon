extern crate osmon;
extern crate structopt;

use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "osmon", about = "Osmon's Compiler")]
pub struct Options {
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}

fn main() {
    println!("{}", "Hello World".to_owned())
}
