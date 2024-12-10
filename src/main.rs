use anyhow::{Ok, Result};
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    println!("{}", url);

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header(USER_AGENT, "machaa at 50_rust")
        .send()
        .await?;

    let users: Vec<User> = res.json().await?;

    println!("{:?}", users);

    Ok(())
}
