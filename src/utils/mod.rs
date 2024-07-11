use colored::Colorize;
use std::{fs::{self, File}, path::Path};
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn download_img_gelbooru(url: String, img_name: String, path: String, proxy: Option<&str>) -> Result<()> {
    let file = format!("{path}/{img_name}");
    if Path::new(&file).exists() {
        println!("Skipping {} because it's already downloaded", file.bright_blue());
        return Ok(());
    }
    println!("Downloading from {} to {}", url.bright_blue(), file.green());
    if !Path::new(&path).exists() {
        fs::create_dir(path).unwrap()
    }
    if let Some(proxy) = proxy {
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy)?)
            .build()?;
        let resp = client.get(url).send().await.expect("request failed");
        let mut out = File::create(file).expect("failed to create file");
        std::io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");

        Ok(())
    } else {
        let resp = reqwest::get(url).await.expect("request failed");
        let mut out = File::create(file).expect("failed to create file");
        std::io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");

        Ok(())
    }
}

pub async fn download_img_boosty(url: String, path: String) -> Result<()> {
    let file: String = if url.contains("?") {
        format!("{}/{}.png", path, url.split_once("image/").unwrap().1.split_once("?").unwrap().0)
    } else {
        format!("{}/{}.png", path, url.split_once("image/").unwrap().1)
    };
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
