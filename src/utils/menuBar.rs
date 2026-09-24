trait MenuUiExt {
    fn close_menu(&mut self);
}

impl MenuUiExt for egui::Ui {
    fn close_menu(&mut self) {
        self.close();
    }
}

pub struct MenuBarState {
    pub show_source_control: bool,
    pub show_repo_picker: bool,
    pub show_about: bool,
}

impl MenuBarState {
    pub fn new() -> Self {
        Self {
            show_source_control: false,
            show_repo_picker: false,
            show_about: false,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        // TODO: wire to your project-creation flow
                        ui.close_menu();
                    }
                    if ui.button("Open Project...").clicked() {
                        // TODO: file dialog (e.g. via `rfd` crate)
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        // TODO: save current project
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Cut").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Copy").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Paste").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Preferences...").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("Asset Importer").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Build Settings").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Scene Hierarchy").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Inspector").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Console").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Utils", |ui| {
                    if ui.checkbox(&mut self.show_source_control, "Source Control").clicked() {
                        ui.close_menu();
                    }
                    if ui.checkbox(&mut self.show_repo_picker, "Clone Repository...").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("Documentation").clicked() {
                        let _ = open::that("https://docs.syrupstudios.lol");
                        ui.close_menu();
                    }
                    if ui.button("Report an Issue").clicked() {
                        let _ = open::that("https://github.com/ItzPancakse/Actinium/issues");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("About Actinium").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });

        if self.show_about {
            egui::Window::new("About Actinium")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Actinium Game Engine");
                    ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                    ui.label("A game engine by Syrup Studios");
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }
    }
}
