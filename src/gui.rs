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

#[derive(Default)]
pub struct GuiState {
    user_commands: Vec<String>,
    disk_actions: Vec<String>,
    disk_percentages: Vec<f32>,
    printer_strings: Vec<String>,
    num_printed_lines: Vec<usize>,
    prints_waiting: usize,
}

impl GuiState {
    pub fn new (num_users: usize, num_disks: usize, num_printers: usize) -> Self {
        let mut user_commands = Vec::new();
        for _ in 0..num_users {
            user_commands.push(String::new());
        }

        let mut disk_actions = Vec::new();
        let mut disk_percentages = Vec::new();
        for _ in 0..num_disks {
            disk_actions.push("Not in use".to_string());
            disk_percentages.push(0.0);
        }

        let mut printer_strings = Vec::new();
        let mut num_printed_lines = Vec::new();
        for _ in 0..num_printers {
            printer_strings.push(String::new());
            num_printed_lines.push(0);
        }

        Self {
            user_commands,
            disk_actions,
            disk_percentages,
            printer_strings,
            num_printed_lines,
            prints_waiting: 0,
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

    fn show_user_data(ui: &mut egui::Ui, id: usize, command: &String) {
        ui.group(|ui| {
            ui.horizontal(|ui| {

                ui.label(format!("User {id}"));

                ui.separator();

                ui.label(command);

            });
        });
    }

    fn show_disk_data(ui: &mut egui::Ui, id: usize, action: String, percentage: f32) {
        ui.group(|ui| {
            ui.horizontal(|ui| {

                ui.label(format!("Disk {id}"));

                ui.separator();

                ui.label(action);

                ui.separator();

                ui.label(format!("{}% full", percentage * 100.0));

            });
        });
    }

    fn show_printer_data(ui: &mut egui::Ui, id: usize, file_string: String, num_lines: usize) {
        ui.group(|ui| {
            ui.horizontal(|ui| {

                ui.label(format!("Printer {id}"));

                ui.separator();

                ui.label(format!("Lines printed: {num_lines}"));

                ui.separator();


                ui.label(format!("{file_string}"));

            });
        });

        // let available_height = ui.available_height() - (500 * num_printers) as f32;
        // let height_per_box = available_height / num_printers as f32;

        // ui.group(|ui| {
        //     ui.vertical(|ui| {
        //         ui.label(format!("Printer {id}"));
        //     });

        //     egui::ScrollArea::vertical()
        //         .max_height(0.0)
        //         .show(ui, |ui| {
        //             ui.add(
        //                 egui::TextEdit::multiline(data)
        //                     .lock_focus(true),
        //             );
        //         });
        // });
    }
}

impl eframe::App for OsGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("Hello Rust GUI!");

        //     ui.label("Enter your name:");
        //     ui.text_edit_singleline(&mut self.name);

        //     if ui.button("Click me").clicked() {
        //         println!("Hello {}", self.name);
        //     }

        //     ui.label(format!("Hello {}", self.name));
        // });

        let state = self.gui_state.lock().unwrap();

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("User Actions");
            for id in 0..state.user_commands.len() {
                OsGui::show_user_data(ui, id, &state.user_commands[id]);
            }
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Printer Info");
            ui.label(format!("Print Jobs Waiting: {}", state.prints_waiting));
            for id in 0..state.printer_strings.len() {
                OsGui::show_printer_data(ui, id, state.printer_strings[id].clone(), state.num_printed_lines[id]);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Disk Info");
            for id in 0..state.disk_percentages.len() {
                OsGui::show_disk_data(ui, id, state.disk_actions[id].clone(), state.disk_percentages[id]);
            }
        });

        ctx.request_repaint();
    }
}