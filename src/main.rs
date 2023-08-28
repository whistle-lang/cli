use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::time::Instant;
use std::{fs, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_wasi::WasiState;

mod config;
mod lsp;
mod util;

use config::Config;

use lsp::WhistleBackend;

use tower_lsp::{LspService, Server};

#[derive(Debug, Parser)]
#[command(name = "whiskey")]
#[command(author = "The Whistle Authors")]
#[command(about = "Next gen Whistle CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// compiles and runs the code
    #[command(arg_required_else_help = true)]
    Run {
        path: String,
    },

    /// compiles the file
    Compile {
        /// input
        #[arg(value_name = "INPUT")]
        path: String,
        /// output file
        #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
        output: Option<String>,
    },

    Build,
    /// launches the language Server
    Lsp,
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

        Commands::Build => {
            let now = Instant::now();
            let text = fs::read_to_string("./Whiskey.toml")
                .expect("Something went wrong, we can't read this file.");
            let config: Config = toml::from_str(text.as_str()).unwrap();
            let text = fs::read_to_string(&config.package.path)
                .expect("Something went wrong, we can't read this file.");
            let bytes = util::compile(&text);
            fs::write(config.package.name + ".wasm", bytes)
                .expect("Something went wrong, we can't write this file.");

            println!(
                "Operation complete! Took us about {} seconds.",
                now.elapsed().as_secs_f64()
            );
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
            tracing_subscriber::fmt().init();
            let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
            let (service, socket) = LspService::new(|client| WhistleBackend {
                client,
                document_map: Arc::new(RwLock::new(HashMap::new())),
            });
            Server::new(stdin, stdout, socket).serve(service).await;
        }
    }
}
