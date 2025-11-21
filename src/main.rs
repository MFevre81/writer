use eframe::egui;
use eframe::egui::Align;

#[derive(Default)]
struct MyApp {
    text: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
         // Menubar at the top
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Creates the horizontal menu bar
            egui::MenuBar::new().ui(ui, |ui| {
                // Adds a menu button named "File"
                ui.menu_button("File", |ui| {
                    // Adds a button in the dropdown menu
                    if ui.button("Quit").clicked() {
                        // Command to close the application
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
        // status bar at the bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{} characters", self.text.len()));
                ui.with_layout(egui::Layout::right_to_left(Align::LEFT), |ui| {
                    ui.label("Ready");
                });
            });
        });

        // central area: text edit filling the remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text).frame(true));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Windows Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}