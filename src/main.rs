pub mod args;
pub mod service;
use std::{fs, io::Write};

use args::{Args, Commands};
use clap::Parser;
use darkdown::converter::converter::Converter;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use service::login_request;

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
            Commands::Logout => println!("Logout"),
            Commands::Convert(converter) => {
                Converter::new().convert_to_html(converter.input.as_str());
            }
        },
        None => {
            let items = vec!["Item 1", "Item 2", "Item 3"];
            let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("Choose your destiny")
                .default(0)
                .items(&items)
                .interact()
                .unwrap();
        }
    }
}

/// Login to the server
pub fn login() {
    // Get the username and password from the user
    let name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Your Username")
        .interact_text()
        .unwrap();
    // Get the password from the user
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Your Password")
        .interact()
        .unwrap();

    let token: String = login_request(name, password);
    // create rusty-leaf.toml file with token
    // check if file exists for all platforms
    let home = std::env::var("HOME").unwrap();
    let if_file_exists = fs::metadata(format!("{}/.rusty-leaf.toml", home));
    if if_file_exists.is_ok() {
        println!("File exists");
        // change the token
        // read the file
        let file = fs::read_to_string(format!("{}/.rusty-leaf.toml", home));
        if file.is_ok() {
            let mut file = file.unwrap();
            file.replace_range(8.., &token);
            fs::write(format!("{}/rusty-leaf.toml", home), file).expect("Unable to write data");
        } else {
            println!("Unable to read file");
        }
    } else {
        let file = fs::File::create(format!("{}/.rusty-leaf.toml", home));
        if file.is_ok() {
            let mut file = file.unwrap();
            file.write_all(format!("token = {}", token).as_bytes())
                .expect("Unable to write data");
        } else {
            println!("Unable to create file");
        }
    }
}
