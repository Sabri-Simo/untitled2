use clap::{Args, Parser, Subcommand};
#[derive(Parser)]
#[command(name = "dockerit")]
#[command(about = "Generate container files automatically")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "generate", alias = "g")]
    Generate {
        #[command(subcommand)]
        subcommand: GenerateSubcommand,
    },
}

#[derive(Subcommand)]
pub enum GenerateSubcommand {
    Dockerfile,
    DockerCompose,
}
