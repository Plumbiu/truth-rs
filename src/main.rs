use clap::{Parser, Subcommand};
use mimalloc::MiMalloc;
use std::{fs, time::Instant};
use truth_rs_constants::log_url;
use truth_rs_core::{json::stringify_json, relation::gen_relations};
use truth_rs_server::start_server;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "truth-rs")]
#[command(about = "A command-line tool for analyzing dependencies under node_moudles")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// [WEB] start web server
    #[command(arg_required_else_help = true)]
    Web {
        /// server listen port
        #[arg(long, short = 'P', default_value_t = 303)]
        port: u16,
    },
    /// [FILE] generate `json` file
    #[command(arg_required_else_help = true)]
    Json {
        /// npm package depth
        #[arg(long, short = 'D', default_value_t = 3)]
        depth: u16,
    },
    /// [FILE] generate `html` file
    #[command(arg_required_else_help = true)]
    Html {
        /// npm package depth
        #[arg(long, short = 'D', default_value_t = 3)]
        depth: u16,
    },
    /// [FILE] generate `txt` file
    #[command(arg_required_else_help = true)]
    Txt {
        /// npm package depth
        #[arg(long, short = 'D', default_value_t = 3)]
        depth: u16,
    },
}

fn main() {
    let start = Instant::now();
    let args = Cli::parse();
    let relations = gen_relations();
    match args.command {
        Commands::Web { port } => {
            log_url(
                &format!("http://localhost:{port}"),
                start.elapsed().as_millis(),
            );
            let _ = start_server(port, relations);
        }
        Commands::Json { depth } => {
            let write_path = "pkgs.json";
            let relations = gen_relations();
            let _ = fs::write(write_path, stringify_json(&relations, depth));
            log_url(write_path, start.elapsed().as_millis());
        }
        Commands::Html { depth } => {
            println!("Cloning {depth}");
        }
        Commands::Txt { depth } => {
            println!("Cloning {depth}");
        }
    }
}
