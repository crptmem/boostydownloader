use std::error::Error;
use imgdl_rs::boosty::auth::Auth;

use clap::{Parser, Subcommand};
use colored::Colorize;

mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Args
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

/// Subcommands
#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Subcommand to download from boosty
    Boosty {
        #[arg(short, long)]
        /// Boosty blog
        blog: String,

        #[arg(short, long)]
        #[clap(default_value_t = String::from("img"))]
        /// Path where images will be saved
        path: String,

        #[arg(short, long)]
        /// Boosty access token
        access_token: Option<String>,

        #[arg(short, long)]
        #[clap(default_value_t = 300)]
        /// Set limit of maximum images to download
        limit: i64,
    },
    /// Subcommand to download from Gelbooru-powered site
    Gelbooru {
        #[arg(short, long)]
        /// Site URL
        url: String,

        #[arg(short, long)]
        #[clap(default_value_t = String::from("img"))]
        /// Path where images will be saved
        path: String,

        #[arg(short, long)]
        /// Gelbooru tags
        tags: String,

        #[arg(long)]
        #[clap(default_value_t = 0)]
        /// Page id
        pid: i64,

        #[arg(long)]
        /// Proxy if requested resource is blocked in your country (SOCKS or HTTP)
        proxy: Option<String>
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.cmd {
        Commands::Boosty { blog, path, access_token, limit } => {
            download_boosty(blog, path, access_token, limit).await;
        },
        Commands::Gelbooru { url, path, tags, pid, proxy } => {
            if let Some(proxy_url) = proxy {
                let proxy_reqwest = reqwest::Proxy::all(proxy_url).unwrap();
                download_gelbooru(url, tags, pid, path, Some(proxy_reqwest)).await;
            } else {
                download_gelbooru(url, tags, pid, path, None).await;
            }
           
        }
    }

    Ok(())
}

async fn download_gelbooru(url: String, tags: String, pid: i64, path: String, proxy: Option<reqwest::Proxy>) {
    println!("Downloading all pictures from tags {} to {}", tags.purple(), path.green());
    let response = imgdl_rs::gelbooru::request::fetch_posts(url.clone(), Some(tags.clone()), Some(pid), proxy.clone()).await.unwrap();
    for i in response.iter() {
        let download_url = format!("{url}/images/{}/{}", i.directory, i.image);
        utils::download_img_gelbooru(
            download_url, i.image.clone(), format!("{path}/{tags}/"), proxy.clone()).await.unwrap();
    }
}

async fn download_boosty(blog: String, path: String, access_token: Option<String>, limit: i64) {
    let auth = access_token.map(Auth::new);
    println!("Downloading all pictures from {} to {}", blog.purple(), path.green());
    let response = imgdl_rs::boosty::request::fetch_posts(blog.clone(), limit, auth.clone()).await.unwrap();
    println!("Total count: {}, limit: {}", response.len(), limit);

    std::fs::create_dir_all(path.clone()).unwrap();
    for i in response.iter() {
        if let Some(data) = i.data.clone() { 
            for content in data.iter() {
                if content.url.is_some() && content.url.clone().unwrap().starts_with("https://images.boosty.to/image/") {
                    utils::download_img_boosty(
                        content.clone().url.clone().unwrap(),
                        path.clone()).await.unwrap();
                }
                if content.url.is_some() && content.content_type.clone() == "ok_video"
                    && content.url.clone().unwrap().contains("id") && content.player_urls.is_some() {
                        let player_urls = content.player_urls.clone().unwrap();
                        for player in player_urls {
                            if player.content_type == "hd" || player.content_type == "full_hd" || player.content_type == "low" {
                                utils::download_video(
                                    player.clone().url, path.clone()).await.unwrap();
                                break;
                            }
                    } 
                }
            }
        }
        for teaser in &i.teaser { 
            if teaser.url.is_some() {
                utils::download_img_boosty(
                    teaser.url.clone().unwrap(),
                    path.clone()).await.unwrap();
            }  
        }
    }
}
