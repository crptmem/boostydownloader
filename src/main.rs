use std::{error::Error, fs, path::Path};
use clap::Parser;
use cli::{Args, Commands};
use imgdl_rs::boosty::auth::Auth;

use futures::future::join_all;
use futures::Future;
use std::pin::Pin;

use colored::Colorize;

mod utils;
mod cli;

type BoxedFuture = Pin<Box<dyn Future<Output = Result<(), utils::Error>> + Send>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.cmd {
        Commands::Boosty { blog, path, access_token, limit, photo_only, video_only } => {
            download_boosty(blog, path, access_token, limit, photo_only, video_only).await;
        },
        Commands::Gelbooru { path, tags, page, all, proxy } => {
            download_gelbooru(tags, page, path, all, proxy).await;
        }
    }

    Ok(())
}

async fn download_gelbooru(tags: String, page: i64, path: String, all: bool, proxy: Option<String>) {
    let path = format!("{path}/{tags}");
    println!("Downloading all pictures from tags {} to {}", tags.purple(), path.green());
    let client = imgdl_rs::gelbooru::request::Client::new(proxy.as_deref()); 
    let attributes = client.fetch_attributes(&tags, page).await.unwrap();
    let page_count = if all { (attributes.count / 100) as i64 } // Divide total post count by post limit per
                                                                // page (100)
    else { 1 };

    println!("Total pages: {page_count} (use `--all` flag to download from all pages)");
    if !Path::new(&path).exists() {
        fs::create_dir(&path).unwrap()
    }

    for p in 0..page_count {
        let response = client.fetch_posts(&tags, p).await.unwrap();
        for i in response.iter() { 
            utils::download_img_gelbooru(
            i.file_url.clone(), i.image.clone(), path.clone(), proxy.clone()).await.unwrap();
        }
    }
}

async fn download_boosty(blog: String, path: String, access_token: Option<String>, limit: i64, photo_only: bool, video_only: bool) {
    let auth = access_token.map(Auth::new);
    println!("Downloading content from {} to {}", blog.purple(), path.green());
    let response = imgdl_rs::boosty::request::Client::fetch_posts(
        blog.clone(), limit, auth.clone()).await.unwrap();
    println!("Total count: {}, limit: {}", response.len(), limit);
    
    std::fs::create_dir_all(path.clone()).unwrap();    
        
    let image_futures: Vec<_> = response.iter().flat_map(|post| {
        // Process data for paid or free posts
        let data_futures: Vec<BoxedFuture> = post.data.as_ref().map_or_else(Vec::new, |data| {
            data.iter().flat_map(|content| {
                let mut futures: Vec<BoxedFuture> = Vec::new();

                if let Some(url) = &content.url {
                    if url.starts_with("https://images.boosty.to/image/") && !video_only {
                        futures.push(Box::pin(utils::download_img_boosty(url.clone(), path.clone())));
                    } else if content.content_type == "ok_video" && !photo_only {
                        if let Some(player_urls) = &content.player_urls {
                            if let Some(player) = player_urls.iter().find(|player| ["hd", "full_hd", "low"].contains(&player.content_type.as_str())) {
                                futures.push(Box::pin(utils::download_video(player.url.clone(), path.clone())));
                            }
                        }
                    }
                }

                futures
            }).collect()
        });

        // Process teaser data for paid posts only if !video_only
        let teaser_futures: Vec<BoxedFuture> = if !video_only {
            post.teaser.iter().filter_map(|teaser| {
                teaser.url.as_ref().map(|url| {
                    Box::pin(utils::download_img_boosty(url.clone(), path.clone())) as Pin<Box<dyn Future<Output = Result<(), utils::Error>> + Send>>
                })
            }).collect()
        } else {
            Vec::new()
        };

        data_futures.into_iter().chain(teaser_futures.into_iter()).collect::<Vec<_>>()
    }).collect();

    join_all(image_futures).await.into_iter().for_each(|result| {
        if let Err(e) = result {
            eprintln!("Error occurred: {:?}", e);
        }
    });
}
