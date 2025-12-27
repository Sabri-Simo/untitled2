mod detection;
use clap::Parser;
use detection::project_type::*;
use detection::version::*;
mod utils;
use detection::dockerfile::*;
use detection::wind::*;
use std::env;
use std::path::PathBuf;
use utils::cli::*;
use utils::file::*;

fn main() {
    /*  #[cfg(windows)]
    prompt_add_to_path();*/
    let cli = Cli::parse();
    match cli.command {
        Command::Generate { subcommand } => match subcommand {
            GenerateSubcommand::Dockerfile => run_dockerit(),
            GenerateSubcommand::DockerCompose => {
                println!("🚧 docker-compose support coming soon");
            }
        },
    }
}
