use std::sync::mpsc::{self, Receiver};
use std::thread;

use serde::Deserialize;

fn build_channel() -> String {
    let Ok(exe_path) = std::env::current_exe() else {
        return "stable".to_string();
    };
    let Some(dir) = exe_path.parent() else {
        return "stable".to_string();
    };

    let candidates = [
        dir.join(".actinium-build-channel"),
        dir.join("../share/actinium/.actinium-build-channel"),
    ];

    for path in candidates {
        if let Ok(contents) = std::fs::read_to_string(&path) {
            return contents.trim().to_string();
        }
    }

    "stable".to_string()
}

pub enum UpdateNotice {
    None,
    Available { version: String, url: String },
    CheckFailed(String),
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    prerelease: bool,
}

const OWNER: &str = "SyrupStudio";
const REPO: &str = "Actinium";

pub fn check_for_update_async(current_version: &str) -> Receiver<UpdateNotice> {
    let (tx, rx) = mpsc::channel();
    let current_version = current_version.to_string();

    thread::spawn(move || {
        if build_channel() == "git" {
            let _ = tx.send(UpdateNotice::None);
            return;
        }

        let notice = check_now(&current_version);
        let _ = tx.send(notice);
    });

    rx
}

fn check_now(current_version: &str) -> UpdateNotice {
    let url = format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest");

    let client = match reqwest::blocking::Client::builder()
        .user_agent("Actinium-UpdateChecker")
        .build()
    {
        Ok(c) => c,
        Err(e) => return UpdateNotice::CheckFailed(e.to_string()),
    };

    let response = match client.get(&url).send() {
        Ok(r) => r,
        Err(e) => return UpdateNotice::CheckFailed(e.to_string()),
    };

    let release: GithubRelease = match response.json() {
        Ok(r) => r,
        Err(e) => return UpdateNotice::CheckFailed(e.to_string()),
    };

    if release.prerelease {
        return UpdateNotice::None;
    }

    let latest = release.tag_name.trim_start_matches('v');

    let (Ok(latest_ver), Ok(current_ver)) = (
        semver::Version::parse(latest),
        semver::Version::parse(current_version),
    ) else {
        return UpdateNotice::CheckFailed("couldn't parse a version string".into());
    };

    if latest_ver > current_ver {
        UpdateNotice::Available {
            version: release.tag_name,
            url: release.html_url,
        }
    } else {
        UpdateNotice::None
    }
}
