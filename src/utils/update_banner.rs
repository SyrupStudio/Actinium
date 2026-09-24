use std::sync::mpsc::Receiver;

use crate::utils::update_checker::{check_for_update_async, UpdateNotice};

pub struct UpdateBanner {
    rx: Option<Receiver<UpdateNotice>>,
    notice: Option<UpdateNotice>,
    dismissed: bool,
}

impl UpdateBanner {
    pub fn new() -> Self {
        Self {
            rx: Some(check_for_update_async(env!("CARGO_PKG_VERSION"))),
            notice: None,
            dismissed: false,
        }
    }

    fn poll(&mut self) {
        if let Some(rx) = &self.rx {
            if let Ok(notice) = rx.try_recv() {
                self.notice = Some(notice);
                self.rx = None;
            }
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.poll();

        if self.dismissed {
            return;
        }

        let Some(UpdateNotice::Available { version, url }) = &self.notice else {
            return;
        };

        egui::Panel::top("update_banner").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("⬆ Update available: {version}"));
                if ui.link("View release").clicked() {
                    let _ = open::that(url);
                }
                if ui.small_button("✕").on_hover_text("Dismiss").clicked() {
                    self.dismissed = true;
                }
            });
        });
    }
}
