use std::{fs, io::Write};
use dialoguer::{theme::ColorfulTheme, Input, Password};
use crate::service::{login_request, create_page};

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

pub fn deploy(file: String, md: bool) {
    println!("Deploying file: {}", file);
    println!("Markdown Support: {}", md);
    let content = fs::read_to_string(file).unwrap();
    let title = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Title")
        .interact_text()
        .unwrap();
    let slug = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Slug")
        .interact_text()
        .unwrap();
    let published = Input::<bool>::with_theme(&ColorfulTheme::default())
        .with_prompt("Published")
        .interact()
        .unwrap();
    create_page(title, content, slug, published);
}

pub fn check_if_token_exists() -> bool {
    let home = std::env::var("HOME").unwrap();
    let if_file_exists = fs::metadata(format!("{}/.rusty-leaf.toml", home));
    if if_file_exists.is_ok() {
        true
    } else {
        false
    }
}

pub fn get_token() -> String {
    if check_if_token_exists() {
        let file = fs::read_to_string(format!("{}/.rusty-leaf.toml", std::env::var("HOME").unwrap()));
        if file.is_ok() {
            let file = file.unwrap();
            let token = file.replace("token = ", "");
            token
        } else {
            println!("Unable to read file");
            String::from("")
        }
    } else {
        println!("Token does not exist");
        String::from("")
    }
}