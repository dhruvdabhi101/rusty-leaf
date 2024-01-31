use std::collections::HashMap;

/// Login Request to the server
pub fn login_request(name: String, password: String) -> String {
    let client = reqwest::blocking::Client::new();
    let url = "http://127.0.0.1:8000/login";
    let mut map = HashMap::new();
    map.insert("username", name);
    map.insert("password", password);
    let res = client.post(url).json(&map).send().unwrap();

    let token = res.json::<HashMap<String, String>>().unwrap();
    return token.get("token").unwrap().to_string();

    // let body = res.text().unwrap();

}
