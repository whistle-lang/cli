use clap::{Parser, Subcommand};
use std::fs;
use std::time::Instant;
use uuid::Uuid;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_wasi::WasiState;

mod lsp;
mod util;

use lsp::WhistleBackend;

use tower_lsp::{LspService, Server};

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

    /// compiles the file
    Compile {
        /// input
        #[arg(value_name = "INPUT")]
        path: String,
        /// output file
        #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
        output: Option<String>,
    },

    /// launches the lsp
    Lsp,

    /// Builds whistle project
    #[command(arg_required_else_help = true)]
    Build { path: String },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run { path } => {
            let text =
                fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
            let bytes = util::compile(&text);
            let mut store = Store::new(Cranelift::default());
            let module = Module::new(&store, bytes).unwrap();
            let wasi_env = WasiState::new(&(Uuid::new_v4().to_string()))
                .finalize(&mut store)
                .unwrap();
            let import_object = wasi_env.import_object(&mut store, &module).unwrap();
            let instance = Instance::new(&mut store, &module, &import_object).unwrap();
            let memory = instance.exports.get_memory("memory").unwrap();
            wasi_env.data_mut(&mut store).set_memory(memory.clone());
            let start = instance.exports.get_function("_start").unwrap();
            start.call(&mut store, &[]).unwrap();
        }
        Commands::Build{ path: _ } => {
            unimplemented!()
        }
        Commands::Compile { path, output } => {
            let now = Instant::now();
            let output = output.unwrap_or(path.replace(".whi", ".wasm"));
            let text =
                fs::read_to_string(path).expect("Something went wrong, we can't read this file.");
            let bytes = util::compile(&text);
            if output.ends_with(".wat") {
                let wasm_text = wasmprinter::print_bytes(&bytes).unwrap();
                fs::write(output, wasm_text.as_bytes())
                    .expect("Something went wrong, we can't write this file.");
            } else {
                fs::write(output, bytes).expect("Something went wrong, we can't write this file.");
            }
            println!(
                "Operation complete! Took us about {} seconds.",
                now.elapsed().as_secs_f64()
            );
        }
        Commands::Lsp => {
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();
            let (service, socket) = LspService::new(|client| WhistleBackend { client });
            Server::new(stdin, stdout, socket).serve(service).await;
        }
    }
}
