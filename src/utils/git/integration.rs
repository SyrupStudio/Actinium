use std::path::Path;
use std::process::Command;

use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Clone)]
pub enum GitProvider {
    GitHub { token: String },
    GitLab { token: String },
    Bitbucket { username: String, app_password: String },
    Gitea { token: String },
    SystemDefault,
}

impl GitProvider {
    fn auth_args(&self) -> Vec<String> {
        let (username, secret) = match self {
            GitProvider::GitHub { token } => ("x-access-token", token.as_str()),
            GitProvider::GitLab { token } => ("oauth2", token.as_str()),
            GitProvider::Gitea { token } => ("token", token.as_str()),
            GitProvider::Bitbucket { username, app_password } => {
                (username.as_str(), app_password.as_str())
            }
            GitProvider::SystemDefault => return vec![],
        };

        let encoded = STANDARD.encode(format!("{username}:{secret}"));
        vec![
            "-c".into(),
            format!("http.extraHeader=Authorization: Basic {encoded}"),
        ]
    }
}

fn run_git(repo_path: &Path, args: &[String]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .map_err(|e| format!("failed to run git (is it installed and on PATH?): {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn clone_repo(remote_url: &str, dest: &Path, provider: &GitProvider) -> Result<(), String> {
    let mut args = provider.auth_args();
    args.push("clone".into());
    args.push(remote_url.into());
    args.push(dest.to_string_lossy().into_owned());
    run_git(Path::new("."), &args).map(|_| ())
}

pub fn pull(repo_path: &Path, provider: &GitProvider) -> Result<(), String> {
    let mut args = provider.auth_args();
    args.push("pull".into());
    run_git(repo_path, &args).map(|_| ())
}

pub fn push(repo_path: &Path, provider: &GitProvider) -> Result<(), String> {
    let mut args = provider.auth_args();
    args.push("push".into());
    run_git(repo_path, &args).map(|_| ())
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: String,
}

pub struct RepoStatus {
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
    pub staged: Vec<FileEntry>,
    pub unstaged: Vec<FileEntry>,
    pub untracked: Vec<FileEntry>,
}

pub fn get_status(repo_path: &Path) -> Result<RepoStatus, String> {
    let out = run_git(
        repo_path,
        &["status".into(), "--porcelain=v1".into(), "--branch".into()],
    )?;
    parse_status(&out)
}

fn parse_status(output: &str) -> Result<RepoStatus, String> {
    let mut lines = output.lines();

    let mut branch = "HEAD".to_string();
    let mut ahead = 0usize;
    let mut behind = 0usize;

    if let Some(branch_line) = lines.next() {
        if let Some(rest) = branch_line.strip_prefix("## ") {
            let branch_part = rest.split("...").next().unwrap_or(rest);
            branch = branch_part.split(' ').next().unwrap_or(branch_part).to_string();

            if let (Some(start), Some(end)) = (rest.find('['), rest.find(']')) {
                for part in rest[start + 1..end].split(", ") {
                    if let Some(n) = part.strip_prefix("ahead ") {
                        ahead = n.parse().unwrap_or(0);
                    }
                    if let Some(n) = part.strip_prefix("behind ") {
                        behind = n.parse().unwrap_or(0);
                    }
                }
            }
        }
    }

    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for line in lines {
        if line.len() < 3 {
            continue;
        }
        let index_status = line.as_bytes()[0] as char;
        let worktree_status = line.as_bytes()[1] as char;
        let path = line[3..].to_string();

        if index_status == '?' && worktree_status == '?' {
            untracked.push(FileEntry { path });
            continue;
        }
        if index_status != ' ' {
            staged.push(FileEntry { path: path.clone() });
        }
        if worktree_status != ' ' {
            unstaged.push(FileEntry { path });
        }
    }

    Ok(RepoStatus {
        branch,
        ahead,
        behind,
        staged,
        unstaged,
        untracked,
    })
}

pub fn stage_path(repo_path: &Path, path: &str) -> Result<(), String> {
    run_git(repo_path, &["add".into(), path.into()]).map(|_| ())
}

pub fn unstage_path(repo_path: &Path, path: &str) -> Result<(), String> {
    run_git(
        repo_path,
        &["restore".into(), "--staged".into(), path.into()],
    )
    .map(|_| ())
}

pub fn diff_unstaged_file(repo_path: &Path, path: &str) -> Result<String, String> {
    run_git(repo_path, &["diff".into(), "--".into(), path.into()])
}

pub fn diff_staged_file(repo_path: &Path, path: &str) -> Result<String, String> {
    run_git(
        repo_path,
        &["diff".into(), "--cached".into(), "--".into(), path.into()],
    )
}
pub fn commit_staged(repo_path: &Path, message: &str) -> Result<(), String> {
    run_git(repo_path, &["commit".into(), "-m".into(), message.into()]).map(|_| ())
}

pub fn list_branches(repo_path: &Path) -> Result<Vec<String>, String> {
    let out = run_git(
        repo_path,
        &["branch".into(), "--format=%(refname:short)".into()],
    )?;
    Ok(out
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

pub fn checkout_branch(repo_path: &Path, name: &str) -> Result<(), String> {
    run_git(repo_path, &["checkout".into(), name.into()]).map(|_| ())
}
