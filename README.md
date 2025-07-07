# Miden GitHub Crawler

Rust script to extract data from selected Miden GitHub repos.

## What it does

- Authenticated GitHub API access
- Fetches:
  - Issues (open & closed)
  - Comments on issues
  - Pull requests (open & closed)
- Saves results as JSON in the `output/` folder

## Setup

1. Create a GitHub personal access token with `repo` scope  
2. Add it to a `.env` file:
   ```
   GITHUB_TOKEN=your_token_here
   ```
3. Run the following in your terminal:
   ```
   cargo run
   ```

## Output

Files are saved in the `output/` folder as:
   ```
    output/
      miden-vm_issues.json
      miden-vm_comments.json
      miden-vm_prs.json
      miden-base_issues.json
      miden-base_comments.json
      miden-base_prs.json
   ```

For now the list of repos is hardcoded in `main.rs`.
