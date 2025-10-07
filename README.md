# GitHub Activity CLI

A command-line tool written in Rust to fetch and display recent GitHub user activity.
https://roadmap.sh/projects/github-user-activity

## Features

- Fetches recent activity from the GitHub API
- Displays activity in a readable format
- Handles various GitHub event types:
  - Push events
  - Issues (opened, closed, etc.)
  - Pull requests
  - Stars (watch events)
  - Forks
  - Repository creation
  - Releases
  - Comments
  - And more!
- Graceful error handling for invalid usernames and API failures
- Clean command-line interface

## Installation

### Prerequisites
- Rust (1.70 or later)
- Cargo (comes with Rust)

### Build from source

1. Clone this repository:
```bash
git clone <repository-url>
cd github-user-activity
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/github-activity` (or `target/release/github-activity.exe` on Windows)

## Usage

### Basic Usage

```bash
github-activity <username>
```

### Examples

```bash
# Fetch activity for a specific user
github-activity kamranahmedse

# Example output:
Recent activity for kamranahmedse:

- Pushed 3 commits to kamranahmedse/developer-roadmap
- Opened issue #123 in kamranahmedse/developer-roadmap
- Starred some-user/awesome-project
- Created branch 'feature-update' in kamranahmedse/developer-roadmap
- Forked awesome-user/cool-project
```

### Help

```bash
github-activity --help
```

## Error Handling

The CLI handles various error scenarios gracefully:

- **Invalid username**: Returns a clear error message
- **User not found**: Displays "User 'username' not found"
- **API rate limit**: Shows rate limit exceeded message
- **Network issues**: Reports connection problems
- **No activity**: Displays "No recent activity found for user: username"

## Supported GitHub Events

The CLI supports and formats the following GitHub event types:

- **PushEvent**: Shows number of commits pushed to a repository
- **CreateEvent**: Repository, branch, or tag creation
- **DeleteEvent**: Branch or tag deletion
- **IssuesEvent**: Issue creation, closure, or updates
- **PullRequestEvent**: Pull request actions
- **WatchEvent**: Repository starring
- **ForkEvent**: Repository forking
- **ReleaseEvent**: Release publishing
- **PublicEvent**: Making repository public
- **MemberEvent**: Adding collaborators
- **IssueCommentEvent**: Comments on issues
- **PullRequestReviewEvent**: Pull request reviews

## Dependencies

- `clap`: Command-line argument parsing
- `reqwest`: HTTP client for API requests
- `tokio`: Async runtime
- `serde`: JSON serialization/deserialization
- `anyhow`: Error handling
- `chrono`: Date/time handling

## API Information

This tool uses the GitHub Events API:
- Endpoint: `https://api.github.com/users/<username>/events`
- Rate limit: 60 requests per hour for unauthenticated requests
- Returns the 30 most recent public events for a user

## License

This project is open source and available under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Development

To run the project in development mode:

```bash
cargo run -- <username>
```

To run tests:

```bash
cargo test
```

To build for release:

```bash
cargo build --release
```