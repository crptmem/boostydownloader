#![feature(string_remove_matches)]
use std::{error::Error, process::exit};

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
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("Downloading all content from {}", args.blog.purple());
    let response = boosty::request::fetch_posts(args.blog.clone()).await?;
    std::fs::create_dir_all(format!("img/{}", args.blog))?;
    for i in response.iter() {
        for teaser in &i.teaser {
            if teaser.url.is_some() {
                utils::download(
                    teaser.url.clone().unwrap(),
                    format!("img/{}/{}.png", args.blog, utils::generate(16))).await?;
            }
        }
    }
    
    Ok(())
}
