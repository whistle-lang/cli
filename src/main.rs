// use std::ffi::OsStr;
// use std::ffi::OsString;
// use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::time::Instant;
use wabt::Wasm2Wat;
mod util;

#[derive(Debug, Parser)]
#[command(name = "whistle")]
#[command(about = "Next gen Whistle CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// compiles and runs the code
    #[command(arg_required_else_help = true)]
    Run { path: String },
    /// checks the code
    Check { path: String },
    /// compiles the file to a wat file
    #[command(arg_required_else_help = true)]
    PrettyCompile {
        /// input
        path: String,
        /// output file
        #[arg(short = 'o')]
        output: String,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run { path } => {
            // TODO: run command
            let now = Instant::now();
            let text =
                fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
            let bytes = util::compile(&text);
            println!("{:#?}", bytes);
            println!(
                "Operation complete! Took us about {} seconds.",
                now.elapsed().as_secs_f64()
            );
        }
        Commands::Check { path } => {
            let now = Instant::now();
            let text =
                fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
            util::check(&text);

            println!(
                "Operation complete! Took us about {} seconds.",
                now.elapsed().as_secs_f64()
            );
        }
        Commands::PrettyCompile { path, output } => {
            let now = Instant::now();
            let text =
                fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
            let bytes = util::compile(&text);
            let wasm_text = String::from_utf8(
                Wasm2Wat::new()
                    .fold_exprs(true)
                    .inline_export(true)
                    .convert(bytes)
                    .unwrap()
                    .as_ref()
                    .to_vec(),
            )
            .unwrap();
            fs::write(output, wasm_text.as_bytes())
                .expect("Something went wrong, we can't write this file.");
            println!(
                "Operation complete! Took us about {} seconds.",
                now.elapsed().as_secs_f64()
            );
        }
    }
}
