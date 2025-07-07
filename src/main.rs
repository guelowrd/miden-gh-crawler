use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::{env, fs};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let token = env::var("GITHUB_TOKEN")?;
    let repos = vec!["0xMiden/miden-base", "0xMiden/miden-vm"];

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("miden-fetcher"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token))?);

    let output_dir = Path::new("output");
    fs::create_dir_all(output_dir)?; // create "output/" if it doesn't exist

    for repo in repos {
        let repo_name = repo.split('/').last().unwrap();

        // === Issues ===
        let issues_url = format!(
            "https://api.github.com/repos/{}/issues?state=all&per_page=100",
            repo
        );
        let issues: Value = client.get(&issues_url).headers(headers.clone()).send().await?.json().await?;
        fs::write(
            output_dir.join(format!("{}_issues.json", repo_name)),
            serde_json::to_string_pretty(&issues)?,
        )?;        
        println!("Saved issues for {}", repo);

        // === Comments ===
        let mut all_comments = Vec::new();
        for issue in issues.as_array().unwrap_or(&vec![]) {
            if let Some(num) = issue["number"].as_u64() {
                let comment_url = format!(
                    "https://api.github.com/repos/{}/issues/{}/comments",
                    repo, num
                );
                let res = client.get(&comment_url).headers(headers.clone()).send().await?;
                let comments: Value = res.json().await?;
                all_comments.push((num, comments));
            }
        }
        fs::write(
            output_dir.join(format!("{}_comments.json", repo_name)),
            serde_json::to_string_pretty(&all_comments)?,
        )?;
        println!("Saved issue comments for {}", repo);

        // === Pull Requests ===
        let prs_url = format!(
            "https://api.github.com/repos/{}/pulls?state=all&per_page=100",
            repo
        );
        let prs: Value = client.get(&prs_url).headers(headers.clone()).send().await?.json().await?;
        fs::write(
            output_dir.join(format!("{}_prs.json", repo_name)),
            serde_json::to_string_pretty(&prs)?,
        )?;
        println!("Saved PRs for {}", repo);
    }

    Ok(())
}
