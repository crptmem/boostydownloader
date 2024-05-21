use colored::Colorize;
use std::{borrow::Borrow, fmt::format, fs::File, path::{Path, PathBuf}};
use error_chain::error_chain;
//use ytd_rs::{YoutubeDL, Arg};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn download(url: String, path: String) -> Result<()> {
    let file = format!("{}/{}.png", path, url.split_once("image/").unwrap().1.split_once("?").unwrap().0);
    if Path::new(&file).exists() {
        println!("Skipping {} because it's already downloaded", file.bright_blue());
        return Ok(());
    }
    println!("Downloading from {} to {}", url.bright_blue(), file.green()); 
    let resp = reqwest::get(url).await.expect("request failed");
    let mut out = File::create(file).expect("failed to create file");
    std::io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");

    Ok(())
}

pub async fn download_video(url: String, path: String) -> Result<()> {
    let file = format!("{}/{}.mp4", path, url.split_once("&id=").unwrap().1);
    if Path::new(&file).exists() {
        println!("Skipping {} because it's already downloaded", file.bright_blue());
        return Ok(());
    }
    println!("Downloading from {} to {}", url.bright_blue(), file.green()); 
    let resp = reqwest::get(url).await.expect("request failed");
    let mut out = File::create(file).expect("failed to create file");
    std::io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");

    Ok(())
}
