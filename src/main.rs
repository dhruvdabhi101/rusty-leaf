pub mod args;
pub mod runner;
pub mod service;

use args::{Args, Commands};
use clap::Parser;
use darkdown::converter::converter::Converter;
use dialoguer::Select;
use runner::{deploy, login, logout};

/// Main function
pub fn main() {
    let args = Args::parse();
    parse_args(args);
}

/// Parse the arguments and call the appropriate function
pub fn parse_args(args: Args) {
    match args.command {
        Some(command) => match command {
            Commands::Login => login(),
            Commands::Logout => logout(),
            Commands::Convert(converter) => {
                Converter::new().convert_to_html(converter.input.as_str());
            }
            Commands::Deploy(deploy_arg) => deploy(deploy_arg.input, deploy_arg.md),
            Commands::Watch(watch_arg) => println!("Watch args {:?}", watch_arg)
        },
        None => {
            let items = vec!["Login", "Logout", "Exit"];
            let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("Choose your destiny")
                .default(0)
                .items(&items)
                .interact()
                .unwrap();
            match selection {
                0 => login(),
                1 => logout(),
                2 => (),
                _ => println!("Error"),
            }
        }
    }
}
