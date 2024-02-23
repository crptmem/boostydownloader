use colored::Colorize;
use rand::Rng;
use std::iter;
use std::fs::File;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdef0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

pub async fn download(url: String, path: String) -> Result<()> {
    let file = format!("{}/{}.png", path, url.split_once("image/").unwrap().1.split_once("?").unwrap().0);
    println!("Downloading from {} to {}", url.bright_blue(), file.green()); 
    let resp = reqwest::get(url).await.expect("request failed");
    let mut out = File::create(file).expect("failed to create file");
    std::io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");

    Ok(())
}
