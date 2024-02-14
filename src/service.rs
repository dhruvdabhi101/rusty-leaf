use std::collections::HashMap;

use colored::{Color, Colorize};

use serde::{Deserialize, Serialize};

use crate::runner::get_token;

#[derive(Debug, Serialize, Deserialize)]
struct PageRequest {
    title: String,
    content: String,
    slug: String,
    published: bool,
}

/// Login Request to the server
pub fn login_request(name: String, password: String) -> String {
    let client = reqwest::blocking::Client::new();
    let url = "http://127.0.0.1:8000/login";
    let mut map = HashMap::new();
    map.insert("username", name);
    map.insert("password", password);
    let res = client.post(url).json(&map).send().unwrap();
    if res.status() != 200 {
        panic!("Login failed");
    }
    let token = res.json::<HashMap<String, String>>().unwrap();
    return token.get("token").unwrap().to_string();
}

pub fn create_page(title: String, content: String, slug: String, published: bool) {
    let client = reqwest::blocking::Client::new();
    let token = get_token();
    let token = make_bearer_token(token);

    let url = "http://127.0.0.1:8000/pages/create-page";
    let map = PageRequest {
        title,
        content,
        slug,
        published,
    };
    // set authorization header
    let res = client
        .post(url)
        .json(&map)
        .header(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&token).unwrap(),
        )
        .send()
        .unwrap();
    if res.status() != 200 {
        panic!("Create Page failed");
    } else {
        println!("{}", "Page created successfully".color(Color::Green));
    }
}

fn make_bearer_token(token: String) -> String {
    let token = format!("Bearer {}", token);
    token
}
