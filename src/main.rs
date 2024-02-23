use std::error::Error;
use boosty_rs::request;

use clap::Parser;
use colored::Colorize;

mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    blog: String,

    #[arg(short, long)]
    path: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = if args.path.is_some() { args.path.unwrap() } else { format!("img/{}", args.blog.clone()) };
    println!("Downloading all content from {} to {}", args.blog.purple(), path.green());
    let response = request::fetch_posts(args.blog.clone()).await?;
    std::fs::create_dir_all(path.clone())?;
    for i in response.iter() {
        for teaser in &i.teaser {
            if teaser.url.is_some() {
                utils::download(
                    teaser.url.clone().unwrap(),
                    path.clone()).await?;
            }
        }
    }
    
    Ok(())
}
