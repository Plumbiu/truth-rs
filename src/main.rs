use clap::{Parser, Subcommand};
use std::{path::Path, time::Instant};
use truth_rs_constants::log_url;
use truth_rs_core::{
    gen_relations, graph::write_graph, json::write_json, tree::write_tree, write_relation,
};
use truth_rs_server::start_server;

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
    Dev {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Web { port } => {
            let start = Instant::now();
            let relations = gen_relations();
            log_url(&format!("http://localhost:{port}"), start.elapsed());
            let _ = start_server(port, relations);
        }
        Commands::Json { depth } => {
            let start = Instant::now();
            let write_path = "pkgs.json";
            write_json(depth, &Path::new(write_path).to_path_buf());
            log_url(write_path, start.elapsed());
        }
        Commands::Html { depth } => {
            println!("Cloning {depth}");
        }
        Commands::Txt { depth } => {
            println!("Cloning {depth}");
        }
        Commands::Dev {} => {
            let write_path = Path::new("packages").join("web").join("public");
            write_graph(&write_path);
            write_relation(&write_path);
            write_tree(4, &write_path);
        }
    }
}
