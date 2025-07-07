use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde_json::Value;
use std::{env, fs};
use std::path::Path;

fn write_markdown_file(path: &std::path::Path, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)
}

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
    let output_json = Path::new("output/json");
    let output_md = Path::new("output/md");
    fs::create_dir_all(output_json)?;
    fs::create_dir_all(output_md)?;


    for repo in repos {
        let repo_name = repo.split('/').last().unwrap();

        // === Issues ===
        let issues_url = format!(
            "https://api.github.com/repos/{}/issues?state=all&per_page=100",
            repo
        );
        let issues: Value = client.get(&issues_url).headers(headers.clone()).send().await?.json().await?;
        //JSON
        fs::write(
            output_json.join(format!("{}_issues.json", repo_name)),
            serde_json::to_string_pretty(&issues)?,
        )?;
        //MD
        let mut issues_md = String::new();
        for issue in issues.as_array().unwrap_or(&vec![]) {
            let number = issue["number"].as_u64().unwrap_or(0);
            let title = issue["title"].as_str().unwrap_or("");
            let body = issue["body"].as_str().unwrap_or("");
            issues_md.push_str(&format!("## Issue #{number}: {title}\n\n{body}\n\n---\n\n"));
        }
        write_markdown_file(
            &output_md.join(format!("{}_issues.md", repo_name)),
            &issues_md,
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
        //JSON
        fs::write(
            output_json.join(format!("{}_comments.json", repo_name)),
            serde_json::to_string_pretty(&all_comments)?,
        )?;
        //MD
        let mut comments_md = String::new();
        for (issue_num, comment_block) in &all_comments {
            comments_md.push_str(&format!("### Comments on Issue #{issue_num}\n\n"));
            for comment in comment_block.as_array().unwrap_or(&vec![]) {
                let user = comment["user"]["login"].as_str().unwrap_or("unknown");
                let body = comment["body"].as_str().unwrap_or("");
                comments_md.push_str(&format!("**@{}**:\n{}\n\n", user, body));
            }
            comments_md.push_str("---\n\n");
        }
        write_markdown_file(
            &output_md.join(format!("{}_comments.md", repo_name)),
            &comments_md,
        )?;
        
        println!("Saved issue comments for {}", repo);

        // === Pull Requests ===
        let prs_url = format!(
            "https://api.github.com/repos/{}/pulls?state=all&per_page=100",
            repo
        );
        //JSON
        let prs: Value = client.get(&prs_url).headers(headers.clone()).send().await?.json().await?;
        fs::write(
            output_json.join(format!("{}_prs.json", repo_name)),
            serde_json::to_string_pretty(&prs)?,
        )?;
        //MD
        let mut prs_md = String::new();
        for pr in prs.as_array().unwrap_or(&vec![]) {
            let number = pr["number"].as_u64().unwrap_or(0);
            let title = pr["title"].as_str().unwrap_or("");
            let body = pr["body"].as_str().unwrap_or("");
            prs_md.push_str(&format!("## PR #{number}: {title}\n\n{body}\n\n---\n\n"));
        }
        write_markdown_file(
            &output_md.join(format!("{}_prs.md", repo_name)),
            &prs_md,
        )?;
        
        println!("Saved PRs for {}", repo);
    }

    Ok(())
}
