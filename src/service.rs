use std::{collections::HashMap, str::FromStr};

use colored::{Color, Colorize};
use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};

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
    println!("{}", token);
    // set authorization header
    let url = "http://127.0.0.1:8000/pages/create-page";
    // let mut map = HashMap::new();
    // map.insert("title", title);
    // map.insert("content", content);
    // map.insert("slug", slug);
    // map.insert("published", published);
    let map = PageRequest {
        title,
        content,
        slug,
        published,
    };
    let temp = reqwest::header::HeaderValue::from_bytes(token.as_bytes()).unwrap();
    println!("{:?}", temp);
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
        println!("{}","Page created successfully".color(Color::Green));
    }
}

fn make_bearer_token(token: String) -> String {
    let token = format!("Bearer {}", token);
    token
}
