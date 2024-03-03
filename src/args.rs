use clap::{command, Parser, Subcommand};

/// simple Args to login and logout
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Commands for the CLI
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[clap(about = "Login to the server")]
    Login,
    #[clap(about = "Logout from the server")]
    Logout,
    #[clap(about = "Convert a file to HTML")]
    Convert(ConvertArgs),
    #[clap(about = "Deploy a file to the server")]
    Deploy(DeployArgs),
    #[clap(about = "Watch test deployment of the page")]
    Watch(WatchArgs),
}

/// Convert Arguments for the CLI
#[derive(Parser, Debug, Clone)]
pub struct ConvertArgs {
    /// Input File
    #[clap(short, long)]
    pub input: String,
    /// Markdown Support
    #[clap(short, long)]
    pub md: bool,
}

/// Deploy Arguments for the CLI
#[derive(Parser, Debug, Clone)]
pub struct DeployArgs {
    /// Input File
    pub input: String,
    /// Markdown Support
    #[clap(short, long)]
    pub md: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct WatchArgs {
    /// Input File
    pub intput: String
}
