use std::error::Error;
use boosty_rs::request;
use boosty_rs::auth::Auth;

use clap::Parser;
use colored::Colorize;

mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    blog: String,

    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    access_token: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let auth: Option<Auth> = if args.access_token.is_some() { Some(Auth::new(args.access_token.unwrap())) } else { None };
    let path = if args.path.is_some() { args.path.unwrap() } else { format!("img/{}", args.blog.clone()) };
    println!("Downloading all content from {} to {}", args.blog.purple(), path.green());
    let response = request::fetch_posts(args.blog.clone(), auth.clone()).await?;
    std::fs::create_dir_all(path.clone())?;
    for i in response.iter() {
        if auth.is_some() {
            for data in &i.data { 
                for content in data.iter() {
                    if content.url.is_some() && content.url.clone().unwrap().starts_with("https://images.boosty.to/image/") {
                        utils::download(
                            content.clone().url.clone().unwrap(),
                            path.clone()).await?;
                    }
                }
            }
        } else {
            for teaser in &i.teaser { 
                if teaser.url.is_some() {
                    utils::download(
                        teaser.url.clone().unwrap(),
                        path.clone()).await?;
                }  
            }
        }
    }
    
    Ok(())
}
