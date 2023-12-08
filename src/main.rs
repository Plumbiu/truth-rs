use clap::{Parser, Subcommand};
use mimalloc::MiMalloc;
use std::{fs, time::Instant};
use truth_rs_constants::log_url;
use truth_rs_core::{json::stringify_json, relation::gen_relations, txt::gen_txt};
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
    Web {
        /// server listen port
        #[arg(long, short = 'P', default_value_t = 8080)]
        port: u16,
        #[arg(long)]
        dev: bool,
    },
    /// [FILE] generate `json` file
    Json {
        /// npm package depth
        #[arg(long, short = 'D', default_value_t = 3)]
        depth: u8,
        #[arg(long)]
        dev: bool,
    },
    /// [FILE] generate `html` file
    Html {
        /// npm package depth
        #[arg(long, short = 'D', default_value_t = 3)]
        depth: u8,
        #[arg(long)]
        dev: bool,
    },
    /// [FILE] generate `txt` file
    Txt {
        /// npm package depth
        #[arg(long, short = 'D', default_value_t = 3)]
        depth: u8,
        #[arg(long)]
        dev: bool,
    },
}

fn main() {
    let start = Instant::now();
    let args = Cli::parse();
    match args.command {
        Commands::Web { port, dev } => {
            let relations = gen_relations(dev);
            log_url(
                &format!("http://localhost:{port}"),
                start.elapsed().as_millis(),
            );
            let _ = start_server(port, relations);
        }
        Commands::Json { depth, dev } => {
            let write_path = "pkgs.json";
            let relations = gen_relations(dev);
            let _ = fs::write(write_path, stringify_json(&relations, depth));
            log_url(write_path, start.elapsed().as_millis());
        }
        Commands::Html { depth, dev } => {
            let write_path = "pkgs.txt";
            let relations = gen_relations(dev);
            let _ = fs::write(write_path, gen_txt(&relations, depth));
            log_url(write_path, start.elapsed().as_millis());
        }
        Commands::Txt { depth, dev } => {
            let write_path = "pkgs.txt";
            let relations = gen_relations(dev);
            let _ = fs::write(write_path, gen_txt(&relations, depth));
            log_url(write_path, start.elapsed().as_millis());
        }
    }
}
