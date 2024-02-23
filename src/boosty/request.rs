extern crate reqwest;

use reqwest::Response;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use error_chain::error_chain;

const API_URL: &str = "https://api.boosty.to";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct CurrencyPrices {
    pub rub: f64,
    pub usd: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Teaser {
    #[serde(rename = "type")]
    pub ctype: String,
    pub width: Option<isize>,
    pub height: Option<isize>,
    pub rendition: Option<String>,
    pub url: Option<String>,
    pub id: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub created_at: u64,
    pub updated_at: Option<u64>,
    pub publish_time: u64,

    pub currency_prices: CurrencyPrices,
    pub teaser: Vec<Teaser>,
    pub show_views_counter: bool,
    pub price: isize,
    pub id: String,
    pub title: String
}

/// Sends a request to Boosty API, returns reqwest Response wrapped in Result
///
/// # Arguments
///
/// * `method` - A string that holds API method, for example blog/blog_name/post
///
/// # Examples
///
/// ```
/// let body = request("blog/boosty/post").await?;
/// let text = body.text().await?;
/// println!("{:?}", text);
/// ```
async fn request(method: String) -> Result<Response> {
    let url = format!("{}/v1/{}", API_URL, method);  // Will result in something like
                                                     // https://api.boosty.to/v1/blog/boosty/post/
                                                     // Trailing slash is required only for
                                                     // fetching all posts, otherwise 404
                                                     // will be returned.
    Ok(reqwest::get(url).await?)
}

/// Fetches all posts from blog, retuns a vector of Post wrapped in Result
///
/// # Arguments
///
/// * `blog_name` - Name of a blog to get posts
///
/// # Examples
/// ```
/// let json = fetch_posts("boosty").await?;
/// println!("{:?}", json); 
/// ```
pub async fn fetch_posts(blog_name: String) -> Result<Vec<Post>> {
    let url = format!("blog/{}/post/", blog_name);
    let json: Value = request(url.clone()).await?.json().await?;
    let posts: Vec<Post> = serde_json::from_value(json["data"].clone()).unwrap();
    Ok(posts)
}

/// Fetch a certain post from blog, retuns Post wrapped in Result
///
/// # Arguments
///
/// * `blog_name` - Name of a blog to get posts
/// * `post_id` - ID of a post in blog
///
/// # Examples
/// ```
/// let post = fetch_post("boosty", "c9fb8a19-c45e-4602-9942-087c3af28c1b").await?;
/// println!("{:?}", post); 
/// ```
pub async fn fetch_post(blog_name: String, post_id: String) -> Result<Post> {
    let url = format!("blog/{}/post/{}", blog_name, post_id);
    let post: Post = request(url).await?.json().await?;
    Ok(post)
}
