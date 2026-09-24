use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use crate::utils::git::integration::{clone_repo, GitProvider};
use crate::utils::git::providers::{github, gitlab};

enum ListState {
    Idle,
    Loading,
    GitHubReady(Vec<github::RepoSummary>),
    GitLabReady(Vec<gitlab::ProjectSummary>),
    Failed(String),
}

pub struct RepoPickerState {
    state: ListState,
    github_rx: Option<Receiver<Result<Vec<github::RepoSummary>, String>>>,
    gitlab_rx: Option<Receiver<Result<Vec<gitlab::ProjectSummary>, String>>>,
    clone_dest: PathBuf,
    status: Option<String>,
}

impl RepoPickerState {
    pub fn new(clone_dest: PathBuf) -> Self {
        Self {
            state: ListState::Idle,
            github_rx: None,
            gitlab_rx: None,
            clone_dest,
            status: None,
        }
    }

    fn poll(&mut self) {
        if let Some(rx) = &self.github_rx {
            if let Ok(result) = rx.try_recv() {
                self.state = match result {
                    Ok(repos) => ListState::GitHubReady(repos),
                    Err(e) => ListState::Failed(e),
                };
                self.github_rx = None;
            }
        }
        if let Some(rx) = &self.gitlab_rx {
            if let Ok(result) = rx.try_recv() {
                self.state = match result {
                    Ok(projects) => ListState::GitLabReady(projects),
                    Err(e) => ListState::Failed(e),
                };
                self.gitlab_rx = None;
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.poll();

        ui.heading("Clone a repository");

        ui.horizontal(|ui| {
            if ui.button("Load my GitHub repos").clicked() {
                if let Some(token) = github::load_token() {
                    self.state = ListState::Loading;
                    self.github_rx = Some(github::list_repos_async(token));
                } else {
                    self.status = Some("Sign in with GitHub first.".into());
                }
            }
            if ui.button("Load my GitLab projects").clicked() {
                let base_url = "https://gitlab.com".to_string();
                if let Some(token) = gitlab::load_token(&base_url) {
                    self.state = ListState::Loading;
                    self.gitlab_rx = Some(gitlab::list_projects_async(base_url, token));
                } else {
                    self.status = Some("Sign in with GitLab first.".into());
                }
            }
        });

        if let Some(status) = &self.status {
            ui.label(status);
        }

        match &self.state {
            ListState::Idle => {}
            ListState::Loading => {
                ui.spinner();
            }
            ListState::Failed(e) => {
                ui.colored_label(egui::Color32::RED, e);
            }
            ListState::GitHubReady(repos) => {
                let repos = repos.iter().map(|r| {
                    (r.full_name.clone(), r.clone_url.clone(), r.private)
                }).collect::<Vec<_>>();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (full_name, clone_url, private) in &repos {
                        ui.horizontal(|ui| {
                            let label = if *private {
                                format!("🔒 {full_name}")
                            } else {
                                full_name.clone()
                            };
                            ui.label(label);
                            if ui.button("Clone").clicked() {
                                self.clone_selected(clone_url, GitProvider::GitHub {
                                    token: github::load_token().unwrap_or_default(),
                                });
                            }
                        });
                    }
                });
            }
            ListState::GitLabReady(projects) => {
                let projects = projects.iter().map(|p| {
                    (p.path_with_namespace.clone(), p.http_url_to_repo.clone(), p.visibility_is_private)
                }).collect::<Vec<_>>();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (path, url, private) in &projects {
                        ui.horizontal(|ui| {
                            let label = if *private {
                                format!("🔒 {path}")
                            } else {
                                path.clone()
                            };
                            ui.label(label);
                            if ui.button("Clone").clicked() {
                                let base_url = "https://gitlab.com".to_string();
                                self.clone_selected(url, GitProvider::GitLab {
                                    token: gitlab::load_token(&base_url).unwrap_or_default(),
                                });
                            }
                        });
                    }
                });
            }
        }
    }

    fn clone_selected(&mut self, url: &str, provider: GitProvider) {
        let dest = self.clone_dest.clone();
        match clone_repo(url, &dest, &provider) {
            Ok(()) => self.status = Some(format!("Cloned into {}", dest.display())),
            Err(e) => self.status = Some(format!("Clone failed: {e}")),
        }
    }
}
