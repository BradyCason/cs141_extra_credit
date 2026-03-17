use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn run_gui(gui_state: Arc<Mutex<GuiState>>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "141 OS Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(OsGui::new(gui_state)))),
    )
}

#[derive(Default, Clone)]
pub struct GuiState {
    user_commands: Vec<String>,
    users_active: Vec<bool>,
    disk_actions: Vec<String>,
    disk_percentages: Vec<f32>,
    disks_active: Vec<bool>,
    printer_strings: Vec<String>,
    num_printed_lines: Vec<usize>,
    prints_waiting: usize,
    printers_active: Vec<bool>,
}

impl GuiState {
    pub fn new (num_users: usize, num_disks: usize, num_printers: usize) -> Self {
        let mut user_commands = Vec::new();
        let mut users_active = Vec::new();
        for _ in 0..num_users {
            user_commands.push(String::new());
            users_active.push(true);
        }

        let mut disk_actions = Vec::new();
        let mut disk_percentages = Vec::new();
        let mut disks_active = Vec::new();
        for _ in 0..num_disks {
            disk_actions.push("Not in use".to_string());
            disk_percentages.push(0.0);
            disks_active.push(true);
        }

        let mut printer_strings = Vec::new();
        let mut num_printed_lines = Vec::new();
        let mut printers_active = Vec::new();
        for _ in 0..num_printers {
            printer_strings.push(String::new());
            num_printed_lines.push(0);
            printers_active.push(true);
        }

        Self {
            user_commands,
            users_active,
            disk_actions,
            disk_percentages,
            disks_active,
            printer_strings,
            num_printed_lines,
            prints_waiting: 0,
            printers_active,
        }
    }

    pub fn update_user(&mut self, id: usize, command: String) {
        self.user_commands[id] = command;
    }

    pub fn update_disk(&mut self, id: usize, action: String, percentage: f32) {
        self.disk_actions[id] = action;
        self.disk_percentages[id] = percentage;
    }

    pub fn update_printer(&mut self, id: usize, file: Option<String>) {
        if let Some(file_name) = file {
            self.num_printed_lines[id] += 1;
            self.printer_strings[id] = format!("Printing file: {file_name}");
        } else {
            self.printer_strings[id] = "Not in use".to_string();
        }
    }

    pub fn increase_prints_waiting(&mut self) {
        self.prints_waiting += 1;
    }

    pub fn decrease_prints_waiting(&mut self) {
        self.prints_waiting -= 1;
    }

    pub fn user_active(&self, user_id: usize) -> bool {
        self.users_active[user_id]
    }

    pub fn set_user_active(&mut self, user_id: usize) {
        self.users_active[user_id] = true;
    }

    pub fn set_user_inactive(&mut self, user_id: usize) {
        self.users_active[user_id] = false;
    }

    pub fn disk_active(&self, disk_id: usize) -> bool {
        self.disks_active[disk_id]
    }

    pub fn set_disk_active(&mut self, disk_id: usize) {
        self.disks_active[disk_id] = true;
    }

    pub fn set_disk_inactive(&mut self, disk_id: usize) {
        self.disks_active[disk_id] = false;
    }

    pub fn printer_active(&self, printer_id: usize) -> bool {
        self.printers_active[printer_id]
    }

    pub fn set_printer_active(&mut self, printer_id: usize) {
        self.printers_active[printer_id] = true;
    }

    pub fn set_printer_inactive(&mut self, printer_id: usize) {
        self.printers_active[printer_id] = false;
    }
}

#[derive(Default)]
struct OsGui {
    gui_state: Arc<Mutex<GuiState>>,
}

impl OsGui {
    fn new(gui_state: Arc<Mutex<GuiState>>) -> Self {
        Self {
            gui_state,
        }
    }

    fn show_user_data(ui: &mut egui::Ui, id: usize, command: &String, gui_state: Arc<Mutex<GuiState>>) {
        ui.group(|ui| {
            ui.horizontal(|ui| {

                ui.label(format!("User {id}"));

                ui.separator();

                ui.label(command);

                ui.separator();

                let active;
                {
                    let gui_state = gui_state.lock().unwrap();
                    active = gui_state.user_active(id);
                }
                if active {
                    if ui.button("Pause").clicked() {
                        let mut gui_state = gui_state.lock().unwrap();
                        gui_state.set_user_inactive(id);
                    }
                } else {
                    if ui.button("Resume").clicked() {
                        let mut gui_state = gui_state.lock().unwrap();
                        gui_state.set_user_active(id);
                    }
                }
            });
        });
    }

    fn show_disk_data(ui: &mut egui::Ui, id: usize, action: String, percentage: f32, gui_state: Arc<Mutex<GuiState>>) {
        ui.group(|ui| {
            ui.horizontal(|ui| {

                ui.label(format!("Disk {id}"));

                ui.separator();

                ui.label(action);

                ui.separator();

                ui.label(format!("{}% full", percentage * 100.0));

                ui.separator();

                let active;
                {
                    let gui_state = gui_state.lock().unwrap();
                    active = gui_state.disk_active(id);
                }
                if active {
                    if ui.button("Pause").clicked() {
                        let mut gui_state = gui_state.lock().unwrap();
                        gui_state.set_disk_inactive(id);
                    }
                } else {
                    if ui.button("Resume").clicked() {
                        let mut gui_state = gui_state.lock().unwrap();
                        gui_state.set_disk_active(id);
                    }
                }
            });
        });
    }

    fn show_printer_data(ui: &mut egui::Ui, id: usize, file_string: String, num_lines: usize, gui_state: Arc<Mutex<GuiState>>) {
        ui.group(|ui| {
            ui.horizontal(|ui| {

                ui.label(format!("Printer {id}"));

                ui.separator();

                ui.label(format!("Lines printed: {num_lines}"));

                ui.separator();

                ui.label(format!("{file_string}"));

                ui.separator();

                let active;
                {
                    let gui_state = gui_state.lock().unwrap();
                    active = gui_state.printer_active(id);
                }
                if active {
                    if ui.button("Pause").clicked() {
                        let mut gui_state = gui_state.lock().unwrap();
                        gui_state.set_printer_inactive(id);
                    }
                } else {
                    if ui.button("Resume").clicked() {
                        let mut gui_state = gui_state.lock().unwrap();
                        gui_state.set_printer_active(id);
                    }
                }

            });
        });
    }
}

impl eframe::App for OsGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state =
        {
            (*self.gui_state.lock().unwrap()).clone()
        };

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("User Actions");
            for id in 0..state.user_commands.len() {
                OsGui::show_user_data(ui, id, &state.user_commands[id], Arc::clone(&self.gui_state));
            }
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Printer Info");
            ui.label(format!("Print Jobs Waiting: {}", state.prints_waiting));
            for id in 0..state.printer_strings.len() {
                OsGui::show_printer_data(ui, id, state.printer_strings[id].clone(), state.num_printed_lines[id], Arc::clone(&self.gui_state));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Disk Info");
            for id in 0..state.disk_percentages.len() {
                OsGui::show_disk_data(ui, id, state.disk_actions[id].clone(), state.disk_percentages[id], Arc::clone(&self.gui_state));
            }
        });

        ctx.request_repaint();
    }
}