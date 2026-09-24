use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::utils::git::integration::{
    checkout_branch, commit_staged, diff_staged_file, diff_unstaged_file, get_status,
    list_branches, pull, push, stage_path, unstage_path, GitProvider, RepoStatus,
};

enum NetworkResult {
    Done,
    Failed(String),
}

pub struct GitPanelState {
    repo_path: PathBuf,
    provider: GitProvider,

    status: Option<RepoStatus>,
    branches: Vec<String>,

    commit_message: String,
    selected_file: Option<String>,
    diff_text: String,

    error: Option<String>,
    network_busy: bool,
    network_rx: Option<Receiver<NetworkResult>>,
}

impl GitPanelState {
    pub fn new(repo_path: PathBuf, provider: GitProvider) -> Self {
        Self {
            repo_path,
            provider,
            status: None,
            branches: Vec::new(),
            commit_message: String::new(),
            selected_file: None,
            diff_text: String::new(),
            error: None,
            network_busy: false,
            network_rx: None,
        }
    }

    pub fn refresh(&mut self) {
        match get_status(&self.repo_path) {
            Ok(status) => {
                self.status = Some(status);
                self.error = None;
            }
            Err(e) => self.error = Some(e),
        }
        self.branches = list_branches(&self.repo_path).unwrap_or_default();
    }

    fn poll_network(&mut self) {
        if let Some(rx) = &self.network_rx {
            if let Ok(result) = rx.try_recv() {
                self.network_busy = false;
                self.network_rx = None;
                match result {
                    NetworkResult::Done => self.refresh(),
                    NetworkResult::Failed(e) => self.error = Some(e),
                }
            }
        }
    }

    fn start_sync(&mut self) {
        if self.network_busy {
            return;
        }
        self.network_busy = true;

        let (tx, rx) = mpsc::channel();
        self.network_rx = Some(rx);

        let repo_path = self.repo_path.clone();
        let provider = self.provider.clone();

        thread::spawn(move || {
            let result = pull(&repo_path, &provider).and_then(|_| push(&repo_path, &provider));
            let _ = tx.send(match result {
                Ok(()) => NetworkResult::Done,
                Err(e) => NetworkResult::Failed(e),
            });
        });
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.poll_network();

        if self.status.is_none() {
            self.refresh();
        }

        ui.horizontal(|ui| {
            ui.heading("Source Control");
            if ui.small_button("⟳").on_hover_text("Refresh").clicked() {
                self.refresh();
            }
        });

        if let Some(err) = &self.error {
            ui.colored_label(egui::Color32::RED, err);
        }

        let Some(status) = &self.status else { return };
        let branch = status.branch.clone();
        let ahead = status.ahead;
        let behind = status.behind;
        let staged = status.staged.clone();
        let unstaged = status.unstaged.clone();
        let untracked = status.untracked.clone();
        let branches = self.branches.clone();

        ui.horizontal(|ui| {
            egui::ComboBox::from_label("")
                .selected_text(format!("🌿 {branch}"))
                .show_ui(ui, |ui| {
                    for name in &branches {
                        if ui.selectable_label(*name == branch, name).clicked() {
                            if let Err(e) = checkout_branch(&self.repo_path, name) {
                                self.error = Some(e);
                            } else {
                                self.refresh();
                            }
                        }
                    }
                });

            let sync_label = if self.network_busy {
                "⏳ Syncing...".to_string()
            } else {
                format!("⇅ Sync ({ahead} ↑ {behind} ↓)")
            };
            if ui
                .add_enabled(!self.network_busy, egui::Button::new(sync_label))
                .clicked()
            {
                self.start_sync();
            }
        });

        ui.separator();

        ui.add(
            egui::TextEdit::multiline(&mut self.commit_message)
                .hint_text("Commit message")
                .desired_rows(3)
                .desired_width(f32::INFINITY),
        );

        let can_commit = !staged.is_empty() && !self.commit_message.trim().is_empty();
        if ui
            .add_enabled(can_commit, egui::Button::new("✔ Commit"))
            .clicked()
        {
            match commit_staged(&self.repo_path, &self.commit_message) {
                Ok(()) => {
                    self.commit_message.clear();
                    self.refresh();
                }
                Err(e) => self.error = Some(e),
            }
        }

        ui.separator();

        let mut action: Option<(String, bool)> = None; // (path, stage_it)

        egui::CollapsingHeader::new(format!("Staged Changes ({})", staged.len()))
            .default_open(true)
            .show(ui, |ui| {
                for file in &staged {
                    ui.horizontal(|ui| {
                        if ui.small_button("−").on_hover_text("Unstage").clicked() {
                            action = Some((file.path.clone(), false));
                        }
                        if ui
                            .selectable_label(
                                self.selected_file.as_deref() == Some(&file.path),
                                &file.path,
                            )
                            .clicked()
                        {
                            self.select_file(&file.path, true);
                        }
                    });
                }
            });

        let change_count = unstaged.len() + untracked.len();
        egui::CollapsingHeader::new(format!("Changes ({change_count})"))
            .default_open(true)
            .show(ui, |ui| {
                for file in unstaged.iter().chain(untracked.iter()) {
                    ui.horizontal(|ui| {
                        if ui.small_button("+").on_hover_text("Stage").clicked() {
                            action = Some((file.path.clone(), true));
                        }
                        if ui
                            .selectable_label(
                                self.selected_file.as_deref() == Some(&file.path),
                                &file.path,
                            )
                            .clicked()
                        {
                            self.select_file(&file.path, false);
                        }
                    });
                }
            });

        if let Some((path, stage_it)) = action {
            let result = if stage_it {
                stage_path(&self.repo_path, &path)
            } else {
                unstage_path(&self.repo_path, &path)
            };
            if let Err(e) = result {
                self.error = Some(e);
            }
            self.refresh();
        }

        if let Some(selected) = self.selected_file.clone() {
            ui.separator();
            ui.label(format!("Diff: {selected}"));
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    ui.monospace(&self.diff_text);
                });
        }
    }

    fn select_file(&mut self, path: &str, staged: bool) {
        self.selected_file = Some(path.to_string());
        let diff = if staged {
            diff_staged_file(&self.repo_path, path)
        } else {
            diff_unstaged_file(&self.repo_path, path)
        };
        self.diff_text = diff.unwrap_or_else(|e| format!("(diff failed: {e})"));
    }
}
