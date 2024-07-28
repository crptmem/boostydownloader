use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Args
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands
}

/// Subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
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

        #[arg(long)]
        #[clap(conflicts_with = "video_only", long = "photo-only")]
        /// Download only photos
        photo_only: bool,
       
        #[arg(long)]
        #[clap(long = "video-only")]
        /// Download only videos
        video_only: bool
    },
    /// Subcommand to download from Gelbooru
    Gelbooru {
        #[arg(short, long)]
        #[clap(default_value_t = String::from("img"))]
        /// Path where images will be saved
        path: String,

        #[arg(short, long)]
        /// Gelbooru tags
        tags: String,

        #[arg(long)]
        #[clap(default_value_t = 0)]
        /// Page
        page: i64,

        #[arg(long)]
        #[clap(default_value_t = false)]
        /// Download images from all pages
        all: bool,

        #[arg(long)]
        /// Proxy if Gelbooru is blocked in your country (SOCKS or HTTP)
        proxy: Option<String>
    }
}

