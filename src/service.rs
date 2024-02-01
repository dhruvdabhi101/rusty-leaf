use std::{collections::HashMap, str::FromStr};

use reqwest::header::AUTHORIZATION;

use crate::runner::get_token;

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
    println!("{}", token);
    // set authorization header
    let url = "http://127.0.0.1:8000/pages/create-page";
    let mut map = HashMap::new();
    map.insert("title", title);
    map.insert("content", content);
    map.insert("slug", slug);
    map.insert("published", published.to_string());
    let res = client.post(url).json(&map).bearer_auth(token).send().unwrap();
    if res.status() != 200 {
        panic!("Create Page failed");
    }
    let page = res.json::<HashMap<String, String>>().unwrap();
    println!("{:?}", page);
}

fn make_bearer_token(token: String) -> String {
    let token = format!("Bearer {}", token);
    token
}

