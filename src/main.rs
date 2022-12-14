use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::time::Instant;
use wabt::Wasm2Wat;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_wasi::WasiState;
use uuid::Uuid;

mod util;

#[derive(Debug, Parser)]
#[command(name = "whiskey")]
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
            let text =
                fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
            let bytes = util::compile(&text);
            let mut store = Store::new(Cranelift::default());
            let module = Module::new(&store, bytes).unwrap();
            let wasi_env = WasiState::new(&(Uuid::new_v4().to_string()))
                .finalize(&mut store).unwrap();
            let import_object = wasi_env.import_object(&mut store, &module).unwrap();
            let instance = Instance::new(&mut store, &module, &import_object).unwrap();
            let memory = instance.exports.get_memory("memory").unwrap();
            wasi_env.data_mut(&mut store).set_memory(memory.clone());
            let start = instance.exports.get_function("_start").unwrap();
            start.call(&mut store, &[]).unwrap();
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
