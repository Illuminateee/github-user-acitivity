use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use reqwest;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "github-activity")]
#[command(about = "A CLI tool to fetch GitHub user activity")]
struct Cli {
    /// GitHub username to fetch activity for
    username: String,
}

#[derive(Debug, Deserialize)]
struct GitHubEvent {
    #[serde(rename = "type")]
    event_type: String,
    actor: Actor,
    repo: Repository,
    payload: serde_json::Value,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct Actor {
    login: String,
}

#[derive(Debug, Deserialize)]
struct Repository {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match fetch_user_activity(&cli.username).await {
        Ok(events) => {
            if events.is_empty() {
                println!("No recent activity found for user: {}", cli.username);
            } else {
                println!("Recent activity for {}:", cli.username);
                println!();
                for event in events {
                    println!("- {}", format_activity(&event));
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn fetch_user_activity(username: &str) -> Result<Vec<GitHubEvent>> {
    let url = format!("https://api.github.com/users/{}/events", username);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "github-activity-cli")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let events: Vec<GitHubEvent> = response.json().await?;
            Ok(events)
        }
        reqwest::StatusCode::NOT_FOUND => {
            Err(anyhow!("User '{}' not found", username))
        }
        reqwest::StatusCode::FORBIDDEN => {
            Err(anyhow!("API rate limit exceeded. Please try again later."))
        }
        status => {
            Err(anyhow!("GitHub API request failed with status: {}", status))
        }
    }
}

fn format_activity(event: &GitHubEvent) -> String {
    match event.event_type.as_str() {
        "PushEvent" => {
            let commits = event.payload.get("commits")
                .and_then(|c| c.as_array())
                .map(|c| c.len())
                .unwrap_or(0);
            format!("Pushed {} commit{} to {}", 
                   commits, 
                   if commits == 1 { "" } else { "s" }, 
                   event.repo.name)
        }
        "CreateEvent" => {
            let ref_type = event.payload.get("ref_type")
                .and_then(|r| r.as_str())
                .unwrap_or("repository");
            match ref_type {
                "repository" => format!("Created repository {}", event.repo.name),
                "branch" => {
                    let branch = event.payload.get("ref")
                        .and_then(|r| r.as_str())
                        .unwrap_or("unknown");
                    format!("Created branch '{}' in {}", branch, event.repo.name)
                }
                "tag" => {
                    let tag = event.payload.get("ref")
                        .and_then(|r| r.as_str())
                        .unwrap_or("unknown");
                    format!("Created tag '{}' in {}", tag, event.repo.name)
                }
                _ => format!("Created {} in {}", ref_type, event.repo.name)
            }
        }
        "DeleteEvent" => {
            let ref_type = event.payload.get("ref_type")
                .and_then(|r| r.as_str())
                .unwrap_or("branch");
            let ref_name = event.payload.get("ref")
                .and_then(|r| r.as_str())
                .unwrap_or("unknown");
            format!("Deleted {} '{}' in {}", ref_type, ref_name, event.repo.name)
        }
        "IssuesEvent" => {
            let action = event.payload.get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("updated");
            let issue_number = event.payload.get("issue")
                .and_then(|i| i.get("number"))
                .and_then(|n| n.as_u64())
                .unwrap_or(0);
            format!("{} issue #{} in {}", 
                   capitalize_first_letter(action), 
                   issue_number, 
                   event.repo.name)
        }
        "PullRequestEvent" => {
            let action = event.payload.get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("updated");
            let pr_number = event.payload.get("number")
                .and_then(|n| n.as_u64())
                .unwrap_or(0);
            format!("{} pull request #{} in {}", 
                   capitalize_first_letter(action), 
                   pr_number, 
                   event.repo.name)
        }
        "WatchEvent" => {
            format!("Starred {}", event.repo.name)
        }
        "ForkEvent" => {
            format!("Forked {}", event.repo.name)
        }
        "ReleaseEvent" => {
            let action = event.payload.get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("published");
            let release_name = event.payload.get("release")
                .and_then(|r| r.get("tag_name"))
                .and_then(|t| t.as_str())
                .unwrap_or("unknown");
            format!("{} release {} in {}", 
                   capitalize_first_letter(action), 
                   release_name, 
                   event.repo.name)
        }
        "PublicEvent" => {
            format!("Made {} public", event.repo.name)
        }
        "MemberEvent" => {
            let action = event.payload.get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("added");
            format!("{} as collaborator to {}", 
                   capitalize_first_letter(action), 
                   event.repo.name)
        }
        "IssueCommentEvent" => {
            let action = event.payload.get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("created");
            let issue_number = event.payload.get("issue")
                .and_then(|i| i.get("number"))
                .and_then(|n| n.as_u64())
                .unwrap_or(0);
            format!("{} comment on issue #{} in {}", 
                   capitalize_first_letter(action), 
                   issue_number, 
                   event.repo.name)
        }
        "PullRequestReviewEvent" => {
            let action = event.payload.get("action")
                .and_then(|a| a.as_str())
                .unwrap_or("submitted");
            let pr_number = event.payload.get("pull_request")
                .and_then(|pr| pr.get("number"))
                .and_then(|n| n.as_u64())
                .unwrap_or(0);
            format!("{} review on pull request #{} in {}", 
                   capitalize_first_letter(action), 
                   pr_number, 
                   event.repo.name)
        }
        _ => {
            format!("Performed {} in {}", event.event_type, event.repo.name)
        }
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
