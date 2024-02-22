extern crate reqwest;

use serde_json::Value;
use error_chain::error_chain;

const API_URL: &str = "https://api.boosty.to";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn fetch_posts(blog_name: String) -> Result<Value> {
    let url = format!("{}/v1/blog/{}/post/", API_URL, blog_name);
    let res = reqwest::get(&url).await?; 
    let body = res.text().await?;
    let v: Value = serde_json::from_str(&body).unwrap();
    Ok(v)
}

