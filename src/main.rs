#![allow(clippy::result_large_err)]

use bulut::machine::Machine;
use osmon::{
    parser::{lex, parse},
    Compiler,
};
use havo::{
    err::MsgWithPos,
    gccjit::Codegen,
    ast::File as ASTFile,
    semantic::*,
    syntax::{ast::*, lexer::reader::Reader},
    Context,
};
use clap::{Parser, Subcommand, Args, ValueEnum};
use std::{fs::File, io::prelude::*, path::PathBuf, process::exit};

/// Hybrid programming language
#[derive(Parser, Debug)]
#[command(name = "osmon")]
#[clap(version, about = "Hybrid programming language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Compile(Compile),
    Run(Run)
}

#[derive(Args, Debug)]
#[clap(version, about = "Compile the project into executable", long_about = None)]
pub struct Compile {
    pub file: PathBuf,
    #[arg(
        long = "opt-level",
        default_value = "2",
        help = "Set optimization level"
    )]
    pub opt_level: u8,
    #[arg(long = "jit", help = "Use JIT compilation instead of AOT compilation")]
    pub jit: bool,
    #[arg(long = "emit-obj", help = "output object file")]
    pub emit_obj: bool,
    #[arg(long = "emit-asm", help = "Print assembly to stdout")]
    pub emit_asm: bool,
    #[arg(
        short,
        long = "output",
        help = "Set output filename"
    )]
    pub output: Option<PathBuf>,
    #[arg(long = "shared", help = "output shared library (.dll or .so)")]
    pub shared: bool,
    #[arg(
        long = "emit-gimple",
        help = "Dump GIMPLE to stdout if gccjit backend used"
    )]
    pub emit_gimple: bool,
    // possible_values = "&[\"gccjit\",\"cranelift\",\"cpp\"]",
    // case_insensitive = true,
    #[arg(
        long = "backend",
        default_value = "gcc-jit",
        value_enum,
        help = "Select backend"
    )]
    pub backend: Backend,
    #[arg(short, long = "link")]
    pub libraries_link: Vec<String>,
    #[arg(short)]
    pub f_gcc_opts: Vec<String>,
    #[arg(
        long = "consteval",
        help = "Enables constant folding and const function evaluating"
    )]
    pub const_eval: bool,
    #[arg(long = "print-ast", help = "Print program")]
    pub print_ast: bool,
    #[arg(
        long = "aggressive-eval",
        help = "try to evaluate normal (not constexpr) functions too"
    )]
    pub aggressive_eval: bool,
}


#[derive(Debug, ValueEnum, Copy, Clone, PartialEq, Eq)]
pub enum Backend {
    GccJIT,
    CPP,
}

impl Backend {
    pub const fn gccjit() -> &'static str {
        "gccjit"
    }

    pub const fn cpp() -> &'static str {
        "cpp"
    }
}

impl std::str::FromStr for Backend {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Backend, &'static str> {
        let s: &str = &s.to_lowercase();
        match s {
            "gccjit" => Ok(Backend::GccJIT),
            "cpp" | "c++" => Ok(Backend::CPP),
            _ => Err("expected gccjit,cpp or cranelift backend"),
        }
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::GccJIT => write!(f, "gccjit"),
            Backend::CPP => write!(f, "cpp"),
        }
    }
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[clap(version, about = "Run the project on runtime mode", long_about = None)]
pub struct Run {
    /// Ishga tushurili kerak bo'lgan ushbu fayl
    #[arg(name = "FAYL")]
    file: Option<PathBuf>,

    /// Qadamlar ko'rsatilishi kerakligi
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), MsgWithPos> {
    let mut src = String::new();

    let args = Cli::parse();

    match args.command {
        Commands::Compile(opts) => {
            let mut file = ASTFile {
                root: opts
                    .file
                    .parent()
                    .unwrap_or(std::path::Path::new(""))
                    .to_str()
                    .unwrap()
                    .to_owned(),
                src: String::new(),
                path: opts.file.to_str().unwrap().to_owned(),
                elems: vec![],
            };
        
            let reader = Reader::from_file(opts.file.to_str().unwrap()).unwrap();
        
            let mut parser = havo::syntax::parser::Parser::new(reader, &mut file);
        
            let err = parser.parse();
            if err.is_err() {
                println!("{}", err.err().unwrap());
                exit(-1);
            }
        
            let mut ctx = Context::new(file);
            ctx.shared = opts.shared;
            ctx.emit_asm = opts.emit_asm;
            ctx.emit_obj = opts.emit_obj;
            ctx.jit = opts.jit;
            ctx.output = opts
                .output
                .map_or(String::new(), |e: PathBuf| e.to_str().unwrap().to_owned());
            ctx.opt = opts.opt_level;
            ctx.gimple = opts.emit_gimple;
            ctx.file.elems.extend(
                opts.libraries_link
                    .iter()
                    .map(|name| Elem::Link(havo::intern(name))),
            );
            let mut semantic = SemCheck::new(&mut ctx);
        
            semantic.run();
        
            if opts.print_ast {
                for elem in ctx.file.elems.iter() {
                    println!("{}", elem);
                }
            }
        
            match opts.backend {
                Backend::CPP => {
                    use havo::ast2cpp::Translator;
                    let mut translator = Translator::new(ctx);
                    translator.run();
                }
                Backend::GccJIT => {
                    let mut cgen = Codegen::new(&mut ctx, "HavoModule");
                    for opt in opts.f_gcc_opts.iter() {
                        cgen.ctx.add_command_line_option(opt);
                    }
                    cgen.compile();
                }
            }
        
            Ok(())
        },
        Commands::Run(ops) => {
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
            
            Ok(())
        }
    }
}
