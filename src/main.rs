#![feature(string_remove_matches)]
use std::io::Error;

use clap::Parser;
use colored::Colorize;

mod boosty;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    blog: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    println!("Downloading all content from {}", args.blog.purple());
    let response = boosty::request::fetch_posts(args.blog).await;
    
    let value = response.unwrap();
    let data = &value["data"].as_array().unwrap();
    for i in 0..data.len() {
        let url = &data[i]["teaser"][0]["url"];
        if !url.is_null() {
            let _ = utils::download(
                url.to_string(),
                format!("{}/{}.png", "img", utils::generate(16))).await;
        }
    }
    Ok(())
}
