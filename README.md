# Miden GitHub Crawler

A simple Rust script to extract issues from Miden GitHub repos.

### Features

- Authenticated GitHub API access
- Fetches all issues (open & closed)
- Dumps structured JSON files per repo

### Usage

1. Create `.env`:

```env
GITHUB_TOKEN=your_github_token_here
