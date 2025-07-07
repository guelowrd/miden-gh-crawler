use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let token = env::var("GITHUB_TOKEN")?;
    let repos = vec!["0xMiden/miden-base", "0xMiden/miden-vm"];

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("miden-fetcher"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token))?);

    for repo in repos {
        let url = format!(
            "https://api.github.com/repos/{}/issues?state=all&per_page=100",
            repo
        );
        let res = client.get(&url).headers(headers.clone()).send().await?;
        let issues: Value = res.json().await?;

        // Write to file
        let repo_name = repo.split('/').last().unwrap();
        let file_path = format!("{}_issues.json", repo_name);
        fs::write(&file_path, serde_json::to_string_pretty(&issues)?)?;
        println!("Saved issues from {} to {}", repo, file_path);
    }

    Ok(())
}
